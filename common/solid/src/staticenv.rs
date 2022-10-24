//! Provides a macro [`staticenv!`] to override the `getenv` function using a
//! static environment variable table.
//!
//! All public items under this module are implementation details of
//! [`staticenv!`].
use core::ptr::null;

/// Define a `getenv` function and related items using a given static
/// environment variable table.
#[macro_export]
macro_rules! staticenv {
    (
        $(
            $( #[$meta:meta] )*
            $name:literal => $value:expr
        ),*
        $(,)?
    ) => {
        const _: () = {
            const PROTO_ENV_TABLE: $crate::staticenv::ProtoEnvTable = &[
                $(
                    $( #[$meta] )*
                    (
                        $crate::staticenv::as_bytes_const($name),
                        $crate::staticenv::as_bytes_const($value),
                    ),
                )*
            ];

            #[deny(unsafe_op_in_unsafe_fn)]
            const _: () = {
                use $crate::staticenv::*;

                const STR_TABLE_LEN: usize = str_table_len(PROTO_ENV_TABLE);
                static STR_TABLE: [u8; STR_TABLE_LEN] = str_table(PROTO_ENV_TABLE);

                static ENV_TABLE: AssertSync<[*const u8; PROTO_ENV_TABLE.len() + 1]> =
                    unsafe { AssertSync::new(env_table(&STR_TABLE)) };

                #[no_mangle]
                static environ: AssertSync<*const *const u8> =
                    unsafe { AssertSync::new(ENV_TABLE.get().as_ptr()) };

                #[no_mangle]
                unsafe extern "C" fn getenv(name: *const u8) -> *const u8 {
                    unsafe { getenv_impl(name, ENV_TABLE.get().as_ptr()) }
                }
            };
        };
    };
}

#[repr(transparent)]
pub struct AssertSync<T>(T);

impl<T> AssertSync<T> {
    pub const unsafe fn new(x: T) -> Self {
        Self(x)
    }

    pub const fn get(&self) -> &T {
        &self.0
    }
}

unsafe impl<T> Sync for AssertSync<T> {}

/// The prototype environment variable table created by [`staticenv!`].
pub type ProtoEnvTable = &'static [(&'static [u8], &'static [u8])];

/// Calculate the required string table size for [`str_table`].
pub const fn str_table_len(proto: ProtoEnvTable) -> usize {
    let mut i = 0;
    let mut len = 0;

    // `for` in `const fn` is unstable
    while i < proto.len() {
        len += proto[i].0.len() + proto[i].1.len() + 2;
        i += 1;
    }

    len
}

/// Compile a string table.
pub const fn str_table<const LEN: usize>(proto: ProtoEnvTable) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    let mut proto_i = 0;
    let mut out_i = 0;

    // `for` in `const fn` is unstable
    while proto_i < proto.len() {
        let (k, v) = proto[proto_i];

        // `k`
        let mut i = 0;
        // Ranged indexing and `copy_from_slice` in `const fn` are unstable
        while i < k.len() {
            assert!(k[i] != 0, "name must not contain null bytes");
            out[out_i] = k[i];
            i += 1;
            out_i += 1;
        }

        // '"="'
        out[out_i] = b'=';
        out_i += 1;

        // `v + "\0"'
        let mut i = 0;
        // Ranged indexing and `copy_from_slice` in `const fn` are unstable
        while i < v.len() {
            assert!(v[i] != 0, "value must not contain null bytes");
            out[out_i] = v[i];
            i += 1;
            out_i += 1;
        }
        out_i += 1;

        proto_i += 1;
    }

    assert!(out_i == LEN);
    out
}

/// Compile an environment table.
pub const fn env_table<const LEN: usize>(str_table: &[u8]) -> [*const u8; LEN] {
    let mut out = [null(); LEN];
    let mut out_i = 0;
    let mut i = 0;

    // `for` in `const fn` is unstable
    while i < str_table.len() {
        // Write the current entry
        out[out_i] = str_table.as_ptr().wrapping_add(i);
        out_i += 1;

        // Move on to the next entry
        while str_table[i] != 0 {
            i += 1;
        }
        i += 1;
    }

    // The resulting table must have exactly one extra element for null
    // termination
    assert!(out_i + 1 == LEN);

    out
}

#[inline]
pub unsafe fn getenv_impl(name: *const u8, mut environ: *const *const u8) -> *const u8 {
    unsafe {
        let name_len = ffi::strlen(name);

        if !ffi::memchr(name, b'=' as _, name_len).is_null() {
            // Invalid variable name
            return null();
        }

        while !(*environ).is_null() {
            let entry: *const u8 = *environ;

            // entry.starts_with(name + "=")
            if ffi::strncmp(name, entry, name_len) == 0 && *entry.add(name_len) == b'=' {
                // Found it
                return entry.add(name_len + 1);
            }

            environ = environ.add(1);
        }
    }

    // Not found
    null()
}

/// A trait for string-like types that can be safely read as `[u8]`.
pub unsafe trait AsBytesConst {}

unsafe impl AsBytesConst for [u8] {}
unsafe impl AsBytesConst for str {}

/// Reinterpret an immutable instance of [`AsBytesConst`] as a `[u8]`.
pub const fn as_bytes_const(x: &(impl AsBytesConst + ?Sized)) -> &[u8] {
    unsafe {
        let len = core::mem::size_of_val(x);
        core::slice::from_raw_parts(x as *const _ as *const u8, len)
    }
}

mod ffi {
    use core::ffi::c_int;

    extern "C" {
        pub fn strlen(st: *const u8) -> usize;
        pub fn memchr(st: *const u8, ch: c_int, count: usize) -> *const u8;
        pub fn strncmp(lhs: *const u8, rhs: *const u8, count: usize) -> c_int;
    }
}

//! High-level binding for SOLID FIlesystem API
use core::{borrow::Borrow, ffi::CStr};
use std::ffi::CString;

/// A trait for converting a value into a SOLID filesystem path.
///
/// This trait is sealed.
///
/// # Safety
///
/// SOLID Filesystem API is not inherently thread-safe. As such, file operations
/// done by safe code must be routed through the thread-safety wrapper by
/// prefixing the paths by `\TS\`. The implementors must ensure that bare paths
/// can not be created from safe code.
pub unsafe trait ToSolidPath: private::Sealed {
    /// The output of [`Self::to_solid_path`].
    type Output<'a>: Borrow<CStr>
    where
        Self: 'a;

    /// Convert `self` to a SOLID filesystem path, adding the
    /// thread-safety wrapper prefix if `Self` is not [`RawPath`].
    fn to_solid_path(&self) -> Option<Self::Output<'_>>;
}

mod private {
    use super::*;
    pub trait Sealed {}
    impl Sealed for &[u8] {}
    impl Sealed for &CStr {}
    impl Sealed for &str {}
    impl Sealed for &std::path::Path {}
    impl<T: AsRef<CStr>> Sealed for RawPath<T> {}
}

// Ideally we want to cover `impl AsRef<Path>`, `impl AsRef<[u8]>`, and
// `impl AsRef<CStr>` simultaneously, which is not possible until we get
// `impl` specialization.

/// `&[u8]` implements this trait by automatically appending the
/// thread-safety wrapper prefix.
// Safety: This impl automatically appends the thread-safety wrapper prefix
unsafe impl ToSolidPath for &[u8] {
    type Output<'a> = CString where Self: 'a;

    fn to_solid_path(&self) -> Option<Self::Output<'_>> {
        if !self.starts_with(br"\") {
            // Relative paths aren't supported
            return None;
        }

        // Apply the thread-safety wrapper
        const SAFE_PREFIX: &[u8] = br"\TS";
        let wrapped_path = [SAFE_PREFIX, *self, &[0]].concat();

        CString::from_vec_with_nul(wrapped_path).ok()
    }
}

/// `&CStr` implements this trait by automatically appending the
/// thread-safety wrapper prefix.
// Safety: This impl automatically appends the thread-safety wrapper prefix
unsafe impl ToSolidPath for &CStr {
    type Output<'a> = CString where Self: 'a;

    #[inline]
    fn to_solid_path(&self) -> Option<Self::Output<'_>> {
        self.to_bytes().to_solid_path()
    }
}

/// `&str` implements this trait by automatically appending the
/// thread-safety wrapper prefix.
// Safety: This impl automatically appends the thread-safety wrapper prefix
unsafe impl ToSolidPath for &str {
    type Output<'a> = CString where Self: 'a;

    #[inline]
    fn to_solid_path(&self) -> Option<Self::Output<'_>> {
        self.as_bytes().to_solid_path()
    }
}

/// `&Path` implements this trait by automatically appending the
/// thread-safety wrapper prefix.
// Safety: This impl automatically appends the thread-safety wrapper prefix
unsafe impl ToSolidPath for &std::path::Path {
    type Output<'a> = CString where Self: 'a;

    #[inline]
    fn to_solid_path(&self) -> Option<Self::Output<'_>> {
        use std::os::solid::ffi::OsStrExt;
        self.as_os_str().as_bytes().to_solid_path()
    }
}

/// Wraps a value to use it as a SOLID filesystem path unmodified.
pub struct RawPath<T>(T);

impl<T> RawPath<T> {
    /// Construct a `RawPath`.
    ///
    /// # Safety
    ///
    /// The passed value is directly used as a SOLID filesystem without
    /// using the thread-safety wrapper. Some filesystem drivers are unsafe
    /// against multi-threaded uses.
    #[inline]
    pub const unsafe fn new(raw_path: T) -> Self {
        Self(raw_path)
    }

    /// Consume a `RawPath<T>` and extract the contained `T`.
    #[inline]
    pub const fn into_inner(self) -> T {
        self.0
    }
}

// Safety: Upheld by the caller of `RawPath::new`
unsafe impl<T: AsRef<CStr>> ToSolidPath for RawPath<T> {
    type Output<'a> = &'a CStr where Self: 'a;

    #[inline]
    fn to_solid_path(&self) -> Option<Self::Output<'_>> {
        Some(self.0.as_ref())
    }
}

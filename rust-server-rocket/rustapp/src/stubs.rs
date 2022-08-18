//! Stub implementations
use std::os::raw::c_char;

#[no_mangle]
static environ: usize = 0;

#[no_mangle]
unsafe extern "C" fn getenv(name: *const c_char) -> *const c_char {
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    let value: &'static [u8] = match name.to_bytes() {
        b"TMPDIR" => b"\\OSCOM_FS\\tmp\0",
        // Increase the default stack size used by `std::thread::spawn`
        // (Debug builds are stack-hungry)
        b"RUST_MIN_STACK" => b"125536\0",
        _ => return std::ptr::null(),
    };
    value.as_ptr() as _
}

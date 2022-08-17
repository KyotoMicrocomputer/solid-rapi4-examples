//! Internal utilities

#[inline]
pub fn abort_on_unwind<R>(f: impl FnOnce() -> R) -> R {
    struct Guard;
    impl Drop for Guard {
        #[inline]
        fn drop(&mut self) {
            panic!("unable to unwind safely from this state");
        }
    }
    let guard = Guard;
    let ret = f();
    core::mem::forget(guard);
    ret
}

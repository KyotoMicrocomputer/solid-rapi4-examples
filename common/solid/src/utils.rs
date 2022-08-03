//! Internal utilities

#[inline]
pub fn may_be_interrupt_context() -> bool {
    // TODO: Is there a SOLID CS API to determine the current context?
    extern "C" {
        fn sns_ctx() -> i32;
    }
    unsafe { sns_ctx() != 0 }
}

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

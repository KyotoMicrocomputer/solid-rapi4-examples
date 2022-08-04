//! Interrupt control
use crate::abi;

pub struct CriticalSection(());

/// Execute closure `f` in an interrupt-free context.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    struct Guard(abi::SOLID_REGISTER);

    let _guard = Guard(unsafe { abi::SOLID_MUTEX_PushInt() });

    impl Drop for Guard {
        #[inline]
        fn drop(&mut self) {
            unsafe { abi::SOLID_MUTEX_PopInt(self.0) };
        }
    }

    f(&CriticalSection(()))
}

/// Enable interrupts.
///
/// # Safety
///
/// Calling this function may interfere with the operations of active critical
/// sections, such as those made by `SOLID_EnterCriticalSection`.
#[inline]
pub unsafe fn enable() {
    // Safety: Upheld by the caller
    unsafe { abi::SOLID_MUTEX_EnaInt() };
}

/// Disable interrupts.
#[inline]
pub fn disable() {
    unsafe { abi::SOLID_MUTEX_DisInt() };
}

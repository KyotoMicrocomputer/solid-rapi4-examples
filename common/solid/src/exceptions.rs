//! Exception handling
use crate::abi;

/// Get a flag indicating whether the current processor is handling an
/// exception, including interrupts and synchronous aborts.
#[inline]
pub fn active() -> bool {
    // Safety: Not unsafe to call
    unsafe { abi::SOLID_VECTOR_IsInInterrupt() }.0 != 0
}

//! Multiprocessing
use crate::abi;

/// Get the current processor ID (zero-based).
#[inline]
pub fn current_processor_id() -> usize {
    unsafe { abi::SOLID_SMP_GetCpuId() }.0 as usize
}

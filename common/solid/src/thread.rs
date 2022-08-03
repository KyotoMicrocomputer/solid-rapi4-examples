//! Thread states
use core::ptr::NonNull;

use crate::abi;

/// A CPU context state ([`abi::SOLID_CPU_CONTEXT`]) passed to an interrupt
/// handler.
pub struct CpuCx<'a> {
    raw: NonNull<abi::SOLID_CPU_CONTEXT>,
    _phantom: core::marker::PhantomData<&'a ()>,
}

unsafe impl Send for CpuCx<'_> {}
unsafe impl Sync for CpuCx<'_> {}

impl CpuCx<'_> {
    #[inline]
    pub(crate) fn new(raw: NonNull<abi::SOLID_CPU_CONTEXT>) -> Self {
        Self {
            raw,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Get a pointer to the underlying `SOLID_CPU_CONTEXT` object.
    #[inline]
    pub fn as_raw(&self) -> NonNull<abi::SOLID_CPU_CONTEXT> {
        self.raw
    }
}

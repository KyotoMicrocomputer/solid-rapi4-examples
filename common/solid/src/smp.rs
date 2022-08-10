//! Multiprocessing
use core::{
    mem::ManuallyDrop,
    ptr::{null_mut, NonNull},
};

use crate::{abi, thread::CpuCx, utils::abort_on_unwind};

/// A set of processors.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProcessorSet {
    bits: u32,
}

impl ProcessorSet {
    /// Construct a `ProcessorSet` comprising of a single processor.
    ///
    /// # Panics
    ///
    /// This function will panic if an invalid processor ID is specified.
    #[inline]
    pub const fn single(processor_id: usize) -> Self {
        assert!(
            processor_id < abi::SOLID_CORE_MAX,
            "processor ID out of range"
        );
        Self::from_bits_truncating(1u32 << processor_id)
    }

    /// Construct a `ProcessorSet` from a bitfield, ignoring non-existent
    /// processors.
    #[inline]
    pub const fn from_bits_truncating(mut bits: u32) -> Self {
        if abi::SOLID_CORE_MAX < u32::BITS as usize {
            bits = bits & ((1 << abi::SOLID_CORE_MAX) - 1);
        }
        Self { bits }
    }

    /// Construct a `ProcessorSet` including all processors in the system.
    #[inline]
    pub const fn all() -> Self {
        Self::from_bits_truncating(u32::MAX)
    }
}

/// Get the current processor ID (zero-based).
#[inline]
pub fn current_processor_id() -> usize {
    unsafe { abi::SOLID_SMP_GetCpuId() }.0 as usize
}

/// Get the number of processors in the system.
#[inline]
pub fn num_processors() -> usize {
    abi::SOLID_CORE_MAX
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum RemoteCallError {
    /// `SOLID_ERR_NOTREADY`
    NotReady,
    /// The current processor ID or an invalid processor ID  was specified as
    /// the target for [`call_on_processor_no_unwind`].
    BadProcessor,
}

/// Call the specified closure on the specified processors.
///
/// The closure will be called in a inter-processor interrupt handler. The
/// execution might be delayed if the target processors have interrupts
/// disabled.
///
/// The closure executions may be serialized if the internal command pool is
/// exhausted.
///
/// Panics in the closure will abort the program.
#[inline]
pub fn call_on_processors_no_unwind<T>(mask: ProcessorSet, f: T) -> Result<(), RemoteCallError>
where
    T: Fn() + Sync,
{
    unsafe extern "C" fn trampoline<T: Fn()>(f: *mut abi::c_void, _: *mut abi::c_void) {
        // Safety: `f` refers to `f` in the parent scope.
        let f = unsafe { &*f.cast::<T>() };
        abort_on_unwind(|| f());
    }

    // Safety:
    // - `T` is safe to call from other processors.
    // - `trampoline` doesn't unwind.
    // - All `trampoline` calls issued here synchronize-with the completion
    //   of this `SOLID_SMP_ForEachCpu` call. (solid-os de9aca526 or later)
    let result = unsafe {
        abi::SOLID_SMP_ForEachCpu(
            Some(trampoline::<T>),
            NonNull::from(&f).cast().as_ptr(),
            null_mut(),
            mask.bits,
        )
    };

    #[cold]
    fn map_err(result: abi::c_int) -> RemoteCallError {
        match result {
            abi::SOLID_ERR_NOTREADY => RemoteCallError::NotReady,
            abi::c_int(e) => panic!("SOLID_SMP_ForEachCPU failed: {e}"),
        }
    }

    if result != abi::SOLID_ERR_OK {
        Err(map_err(result))
    } else {
        Ok(())
    }
}

/// Call the specified closure on the specified processors, propagating panics
/// to the caller.
///
/// The closure will be called in a inter-processor interrupt handler. The
/// execution might be delayed if the target processors have interrupts
/// disabled.
///
/// The closure executions may be serialized if the internal command pool is
/// exhausted.
///
/// If more than one closure calls panic, all but the first one will be
/// discarded.
#[inline]
pub fn call_on_processors<T>(mask: ProcessorSet, f: T) -> Result<(), RemoteCallError>
where
    T: Fn() + Sync,
{
    // TODO: Optimize for `cfg(panic = "abort")`
    use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
    use sync_wrapper::SyncWrapper;
    let caught_panic_cell = once_cell::sync::OnceCell::new();
    {
        let caught_panic_cell = &caught_panic_cell;
        call_on_processors_no_unwind(mask, move || {
            if let Err(e) = catch_unwind(AssertUnwindSafe(&f)) {
                // Store the information of the caught panic. Keep only the first
                // one and discard the others.
                let _ = caught_panic_cell.set(SyncWrapper::new(e));
            }
        })?;
    }
    if let Some(caught_panic) = caught_panic_cell.into_inner() {
        resume_unwind(caught_panic.into_inner());
    }
    Ok(())
}

/// Call the specified closure on the specified processor.
///
/// The closure will be called in a inter-processor interrupt handler. The
/// execution might be delayed if the target processor has interrupts
/// disabled.
///
/// Panics in the closure will abort the program.
#[inline]
pub fn call_on_processor_no_unwind<T, R>(processor_id: usize, f: T) -> Result<R, RemoteCallError>
where
    T: FnOnce(CpuCx<'_>) -> R + Send,
    R: Send,
{
    if processor_id >= num_processors() {
        return Err(RemoteCallError::BadProcessor);
    }

    union St<T, R> {
        f: ManuallyDrop<T>,
        r: ManuallyDrop<R>,
    }

    unsafe extern "C" fn trampoline<T: FnOnce(CpuCx<'_>) -> R, R>(
        st: *mut abi::c_void,
        cpu_cx: *mut abi::c_void,
    ) {
        abort_on_unwind(|| {
            // Safety: `st` refers to `st` in the parent scope.
            let st = unsafe { &mut *st.cast::<St<T, R>>() };
            // Safety: `f` is the active field of `st`.
            let f = unsafe { ManuallyDrop::take(&mut st.f) };
            let cpu_cx = CpuCx::new(NonNull::new(cpu_cx).expect("null cpu context").cast());
            st.r = ManuallyDrop::new(f(cpu_cx));
        });
    }

    let mut st = St {
        f: ManuallyDrop::new(f),
    };

    // Safety:
    // - `T` is safe to mutate from a different processor.
    // - `R` is safe to send back to the current processor.
    // - `trampoline` doesn't unwind.
    // - The `trampoline` call issued here synchronize-with the completion
    //   of this `SOLID_SMP_ForEachCpu` call. (solid-os 2482e4ddd or later)
    let result = unsafe {
        abi::SOLID_SMP_RequestExec(
            abi::c_int(processor_id as _),
            Some(trampoline::<T, R>),
            NonNull::from(&mut st).cast().as_ptr(),
            null_mut(), // ignored because of `A2CONTEXT`
            abi::c_int(abi::SOLID_SMP_REQFLAG_A2CONTEXT as _),
        )
    };

    #[cold]
    fn map_err(result: abi::c_int) -> RemoteCallError {
        match result {
            abi::SOLID_ERR_NOTREADY => RemoteCallError::NotReady,
            abi::SOLID_ERR_PAR => RemoteCallError::BadProcessor,
            abi::c_int(e) => panic!("SOLID_SMP_RequestExec failed: {e}"),
        }
    }

    if result != abi::SOLID_ERR_OK {
        // Safety: `f` is still the active field of `st`.
        unsafe { ManuallyDrop::into_inner(st.f) };
        Err(map_err(result))
    } else {
        // Safety: `r` is now the active field of `st`.
        Ok(unsafe { ManuallyDrop::into_inner(st.r) })
    }
}

/// Call the specified closure on the specified processor, propagating any panic
/// to the caller.
///
/// The closure will be called in a inter-processor interrupt handler. The
/// execution might be delayed if the target processor has interrupts
/// disabled.
#[inline]
pub fn call_on_processor<T, R>(processor_id: usize, f: T) -> Result<R, RemoteCallError>
where
    T: FnOnce(CpuCx<'_>) -> R + Send,
    R: Send,
{
    // TODO: Optimize for `cfg(panic = "abort")`
    use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
    Ok(call_on_processor_no_unwind(processor_id, move |cx| {
        catch_unwind(AssertUnwindSafe(|| f(cx)))
    })?
    .unwrap_or_else(|e| resume_unwind(e)))
}

//! Multiprocessing
use core::{
    mem::ManuallyDrop,
    ptr::{null_mut, NonNull},
};

use crate::{abi, thread::CpuCx, utils::abort_on_unwind};

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

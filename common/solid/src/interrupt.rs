//! Interrupt control
//!
//! # External Code Assumptions
//!
//! External code does not cause racy memory accesses. For example, if an
//! application performs operation `R` in which Rust code attempts to register
//! an interrupt handler for a particular interrupt line by
//! [`Handler::register_static`] and another operation `E` in which external
//! code attempts to register an interrupt handler for the same interrupt line
//! without using `Handler` (e.g., by calling `SOLID_INTC_Register` directly),
//! either `R` or `E` must happen after another.
//!
//! External code may enable an interrupt line by calling `SOLID_INTC_Enable`,
//! but the resultant handler executions must synchronize-with the last handler
//! registration ([`Handler::register`]`[_static]`) for that interrupt line.
use core::{
    cell::UnsafeCell,
    pin::Pin,
    ptr::{null_mut, NonNull},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{abi, smp, thread::CpuCx, utils::abort_on_unwind};

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

/// A handler function for [`Handler`].
///
/// This trait is sealed; it can not be implemented externally.
///
/// `Send`: The handler will run with no synchronization with the creator
/// thread.
///
/// `'static`: A registered handler will remain registered even if all
/// references to the [`Handler`] are removed. (This is allowed by the pinning
/// guarantees.)
///
/// # Processor states
///
/// The handler is called in an interrupt context with interrupts enabled.
pub trait HandlerFn: Send + private::Sealed + 'static {
    /// Call the interrupt handler.
    ///
    /// # Safety
    ///
    /// This method can only be called from a SOLID interrupt handler. It's in
    /// general unsafe to call from user code.
    unsafe fn call(self: Pin<&mut Self>, cx: CpuCx<'_>);
}

/// The implementation of `HandlerFn` for closures.
///
/// `Unpin`: `FnMut` requires an unpinned receiver.
impl<T: FnMut(CpuCx<'_>) + Unpin + Send + 'static> HandlerFn for T {
    #[inline]
    unsafe fn call(mut self: Pin<&mut Self>, cx: CpuCx<'_>) {
        // Re-enable interrupts.
        // Safety: We are not inside a SOLID critical section.
        unsafe { enable() };

        (*self)(cx);
    }
}

impl<T: HandlerFn> HandlerFn for Option<T> {
    #[inline]
    unsafe fn call(self: Pin<&mut Self>, cx: CpuCx<'_>) {
        if let Some(inner) = self.as_pin_mut() {
            // Safety: Upheld by the caller
            unsafe { inner.call(cx) }
        }
    }
}

mod private {
    use super::*;
    /// Sealed trait pattern (prevents external implementations of
    /// `HandlerFn`)
    pub trait Sealed {}
    impl<T: FnMut(CpuCx<'_>) + Unpin + Send + 'static> Sealed for T {}
    impl<T: HandlerFn> Sealed for Option<T> {}
}

/// Options for [`Handler::register`].
#[derive(Clone, Debug)]
pub struct HandlerOptions {
    intno: Number,
    priority: i32,
    config: i32,
    processor_set: smp::ProcessorSet,
}

impl HandlerOptions {
    /// Construct a `HandlerOptions` with default option values and the
    /// specified interrupt number and priority.
    #[inline]
    pub const fn new(intno: Number, priority: i32) -> Self {
        Self {
            intno,
            priority,
            config: -1,
            processor_set: smp::ProcessorSet::single(0),
        }
    }

    /// Update `self` to configure the interrupt line as edge-triggered.
    #[inline]
    pub const fn with_edge_triggered(self) -> Self {
        Self {
            config: 0b10,
            ..self
        }
    }

    /// Update `self` to configure the interrupt line as level-sensitive.
    #[inline]
    pub const fn with_level_triggered(self) -> Self {
        Self {
            config: 0b10,
            ..self
        }
    }

    /// Update `self` to target the specified processor.
    ///
    /// # Panics
    ///
    /// This function will panic if an invalid processor ID is specified.
    #[inline]
    pub const fn with_target_processor(self, processor_id: usize) -> Self {
        self.with_target_processor_set(smp::ProcessorSet::single(processor_id))
    }

    /// Update `self` to target the specified processors.
    #[inline]
    pub const fn with_target_processor_set(self, processor_set: smp::ProcessorSet) -> Self {
        Self {
            processor_set,
            ..self
        }
    }
}

/// The error type for [`Handler::register`].
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum RegisterError {
    /// There is already an interrupt handler registered for `(intno,
    /// processor_id)` (PPI) or `intno` (SPI).
    InterruptLineAlreadyHasHandler,
    /// The parameters of the interrupt handler are incorrect. For example, the
    /// priority is out of range, or the target processor set includes a
    /// processor other than the first eight ones.
    BadParam,
}

/// The safe wrapper for a SOLID-OS interrupt handler.
pub struct Handler<T: HandlerFn> {
    inner: abi::SOLID_INTC_HANDLER,
    /// The interrupt line for which the handler is registered.
    line: Line,
    handler: UnsafeCell<T>,
    /// `SOLID_INTC_HANDLER` must remain in place as long as the timer is
    /// active.
    _pin: core::marker::PhantomPinned,
}

/// `Handler` can be controlled by any threads, though the operations may fail at
/// runtime if the preconditions are not satisfied.
unsafe impl<T: HandlerFn> Send for Handler<T> {}
/// `&Handler` permits no operations, hence this is safe.
unsafe impl<T: HandlerFn> Sync for Handler<T> {}

/// The destructor for [`Handler`].
///
/// # Panics
///
/// **`drop` will abort if fails to unregister the handler.** The possible
/// causes are:
///
/// - The handler's [`Number`] refers to a PPI, and the handler was registered
///   on a different processor than the current one.
impl<T: HandlerFn> Drop for Handler<T> {
    fn drop(&mut self) {
        abort_on_unwind(|| {
            // Safety: `drop` can always do this safely.
            let this = unsafe { Pin::new_unchecked(self) };
            // Safety: Upheld by the caller of [`Self::register`]
            unsafe { this.unregister() };
        });
    }
}

impl<T: HandlerFn> Handler<T> {
    /// Construct an unregistered `Handler`.
    #[inline]
    pub const fn new(handler: T) -> Self {
        Self {
            inner: abi::SOLID_INTC_HANDLER {
                // zeroed to allow placing it in .bss
                intno: 0,
                priority: 0,
                config: 0,
                func: null_mut(),
                param: null_mut(),
            },
            line: Line(0),
            handler: UnsafeCell::new(handler),
            _pin: core::marker::PhantomPinned,
        }
    }

    /// The outer interrupt handler.
    unsafe extern "C" fn handler_trampoline(
        param: *mut u8,
        cpu_cx: *mut abi::SOLID_CPU_CONTEXT,
    ) -> abi::c_int {
        abort_on_unwind(|| {
            // Safety: `param`'s value is taken from the corresponding handler
            // object's `SOLID_INTC_HANDLER::param`. `Self::register` derives
            // this value by casting `&Self`. Since `Self::register` takes a
            // pinned receiver, user code can not invalidate `*this` without our
            // code intervening. Our `Self::drop` makes sure that all handler
            // calls are complete before the storage of `*this` is reclaimed for
            // other uses.
            let this = unsafe { &*param.cast::<Self>() };

            let cpu_cx = NonNull::new(cpu_cx).expect("null cpu context");
            let cpu_cx = CpuCx::new(cpu_cx);

            // Safety: When the handler is registered, we the handler have the
            // exclusive ownership of `T`. The handler runs on one processor at
            // once (on GICv2, this is guaranteed by the GIC hardware).
            let handler = unsafe { &mut *this.handler.get() };
            // Safety: `Self` is `!Unpin`, and we maintain the pinning
            // guarantees of `this.handler`
            let handler = unsafe { Pin::new_unchecked(handler) };
            // Safety: We are calling it from a SOLID interrupt handler, which
            // is allowed to do this
            unsafe { handler.call(cpu_cx) };

            abi::SOLID_ERR_OK
        })
    }

    /// Check if the interupt handler is registered.
    #[inline]
    pub fn is_registered(&self) -> bool {
        !self.inner.param.is_null()
    }

    /// Get the interrupt number.
    #[inline]
    pub fn number(&self) -> Number {
        Number(self.inner.intno)
    }

    /// Register the interrupt handler that lives throughout the program's
    /// lifetime.
    ///
    /// See [`Self::register`] for semantics.
    #[inline]
    pub fn register_static(
        self: Pin<&'static mut Self>,
        options: &HandlerOptions,
    ) -> Result<bool, RegisterError> {
        // Safety: `*self` is never dropped, hence the unsafe destructor will
        // never run
        unsafe { self.register(options) }
    }

    /// Register the interrupt handler.
    ///
    /// On a successful registeration, the interrupt line specified by the
    /// [`Number`] associated with `self` will have the following attributes
    /// automatically configured:
    ///
    /// - Priority: Set to the value passed to [`Self::new`].
    /// - Target processor set: The first processor.
    ///
    /// # Safety
    ///
    /// The destructor of `Handler` will unregister the interrupt handler in an
    /// unsafe manner. It is up to the caller to ensure that the unregistration
    /// happens after all handler invocations. Calling [`Number::disable`] or
    /// `SOLID_INTC_Disable` may not be sufficient.
    ///
    /// If you have a `'static` reference to `Self`, consider using
    /// [`Self::register_static`] instead.
    pub unsafe fn register(
        self: Pin<&mut Self>,
        &HandlerOptions {
            intno,
            priority,
            config,
            ref processor_set,
        }: &HandlerOptions,
    ) -> Result<bool, RegisterError> {
        // Safety: We preserve the pinning invariants of the contained values
        let this = unsafe { Pin::get_unchecked_mut(self) };

        // Already registered?
        if this.is_registered() {
            return Ok(false);
        }

        // Disable interrupts to prevent processor migration and prevent
        // deadlock while doing `line.lock()`
        free(|_| {
            let line =
                Line::from_intno_for_current_processor(intno).ok_or(RegisterError::BadParam)?;
            let _line_guard = line.lock();

            // `SOLID_INTC_RegisterWithTargetProcess`'s definition restricts the
            // selectable processors to the first eight ones in the system
            let processor_set: i8 = u8::try_from(processor_set.as_u32_bits())
                .map_err(|_| RegisterError::BadParam)? as i8;

            this.inner = abi::SOLID_INTC_HANDLER {
                intno: intno.0,
                priority,
                config,
                // TODO: This is the only thing that differs between
                // the instantiations of this function with different `T`s.
                // Reduce the monomorphization cost
                func: Self::handler_trampoline as _,
                // Since `*self` is pinned, we know its address is stable until
                // `Self::drop` is called.
                param: &*this as *const _ as *mut _,
            };
            debug_assert!(this.is_registered());

            // Register `self.inner` to the system interrupt handler table
            // (`InterruptController::m_p[pi]Handler`)
            //
            // Safety:
            // - The interrupt line is disabled. (If it's enabled, this call
            //   doesn't properly synchronize with the resultant interrupt
            //   handler executions on other processors, leading to UB.)
            //   External code is assumed to uphold this as per the external
            //   code assumption.
            // - `_line_guard` protects us against dead races on `m_pHandler`.
            // - `inner.param` is initialized properly.
            // - External code assumption: External code does not cause racy
            //   memory accesses by making conflicting `SOLID_INTC_Register`
            //   calls.
            let result = unsafe {
                abi::SOLID_INTC_RegisterWithTargetProcess(
                    (&this.inner) as *const _ as *mut _,
                    processor_set,
                )
            };
            if result != abi::SOLID_ERR_OK {
                // Undo the effect on error
                this.inner.param = null_mut();
            }
            match result {
                abi::SOLID_ERR_OK => {}
                abi::SOLID_ERR_PAR => return Err(RegisterError::BadParam),
                abi::SOLID_ERR_ALREADYUSED => {
                    return Err(RegisterError::InterruptLineAlreadyHasHandler)
                }
                abi::c_int(e) => panic!("SOLID_INTC_Register failed: {e}"),
            }

            // Allow enabling the interrupt line
            this.line = line;

            Ok(true)
        })
    }

    /// Unregister the interrupt handler.
    ///
    /// # Safety
    ///
    /// See [`Self::register`].
    #[inline]
    unsafe fn unregister(self: Pin<&mut Self>) -> bool {
        // Safety: We preserve the pinning invariants of the contained values
        let this = unsafe { Pin::get_unchecked_mut(self) };

        // Not registered?
        if !this.is_registered() {
            return false;
        }

        // Disable interrupts to prevent processor migration and prevent
        // deadlock while doing `line.lock()`
        free(|_| {
            let line = Line::from_intno_for_current_processor(Number(this.inner.intno))
                .expect("invalid intno");
            assert_eq!(
                line, this.line,
                "attempted to unregister PPI handler from a different processor \
                than the one for which the handler was registered"
            );

            let _line_guard = line.lock();

            // Safety:
            // - The caller ensures that all handler executions made with
            //   `this.inner` happen before the current `unregister` call.
            let result = unsafe { abi::SOLID_INTC_UnRegister((&this.inner) as *const _ as *mut _) };
            match result {
                abi::SOLID_ERR_OK => {}
                abi::c_int(e) => panic!("SOLID_INTC_UnRegister failed: {e}"),
            }

            this.inner.param = null_mut();

            true
        })
    }
}

/// The error type for [`Number::enable`].
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum EnableError {
    /// No handler is registered by [`Handler::register`] for the interrupt
    /// line.
    NoHandler,
    /// The interrupt number is invalid.
    BadParam,
}

pub use EnableError as DisableError;

/// An interrupt number.
///
/// Note that, for PPIs (`< 32`), a `Number` corresponds to different interrupt
/// lines depending on the current processor.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Number(pub i32);

impl Number {
    /// Enable the interrupt line.
    ///
    /// A handler must be registered by [`Handler::register`]`[_static]` for
    /// the interrupt line.
    ///
    /// # Memory Ordering
    ///
    /// If the method call enables the interrupt line, the call
    /// synchronizes-with all resultant handler calls.
    pub fn enable(self) -> Result<(), EnableError> {
        // Disable interrupts to prevent processor migration and prevent
        // deadlock while doing `line.lock()`
        free(|_| {
            let line = Line::from_intno_for_current_processor(self).ok_or(EnableError::BadParam)?;
            let _line_guard = line.lock();

            match () {
                // Make all preceding memory accesses (which includes
                // `SOLID_INTC_Register` because of the ordering enforced by
                // `line.lock`) visible to the handler.
                #[cfg(target_arch = "aarch64")]
                () => unsafe { core::arch::asm!("dsb ish") },
            }

            // Safety:
            // - The `SOLID_INTC_Register` call for this interrupt line's
            //   handler happens before all handler calls resulting from this
            //   `SOLID_INTC_Enable` call.
            // - Safe code cannot unregister a handler. Therefore, there exist
            //   no other `SOLID_INTC_Register` calls for this interrupt line.
            unsafe { self.enable_raw() }
        })
    }

    /// Enable the interrupt line.
    ///
    /// A handler must be registered by [`Handler::register`]`[_static]` for
    /// the interrupt line.
    ///
    /// # Memory Ordering
    ///
    /// If the method call enables the interrupt line, the call
    /// synchronizes-with all resultant handler calls.
    pub fn disable(self) -> Result<(), EnableError> {
        // Disable interrupts to prevent processor migration and prevent
        // deadlock while doing `line.lock()`
        free(|_| {
            let line = Line::from_intno_for_current_processor(self).ok_or(EnableError::BadParam)?;
            let _line_guard = line.lock();

            // Safety: See `enable` above.
            unsafe { self.disable_raw() }
        })
    }

    /// Enable the interrupt line.
    ///
    /// A handler must be registered for the interrupt line.
    ///
    /// # Safety
    ///
    /// Data race may occur if this call and a handler (un)registeration for
    /// the specified interrupt line are not synchronized with each other.
    pub unsafe fn enable_raw(self) -> Result<(), EnableError> {
        // Safety: Upheld by the caller
        match unsafe { abi::SOLID_INTC_Enable(abi::c_int(self.0)) } {
            abi::SOLID_ERR_OK => Ok(()),
            abi::SOLID_ERR_PAR => Err(EnableError::BadParam),
            abi::SOLID_ERR_NOTUSED => Err(EnableError::NoHandler),
            abi::c_int(e) => panic!("SOLID_INTC_Enable failed: {e}"),
        }
    }

    /// Disable the interrupt line.
    ///
    /// A handler must be registered for the interrupt line.
    ///
    /// # Safety
    ///
    /// Data race may occur if this call and a handler (un)registeration for
    /// the specified interrupt line are not synchronized with each other.
    pub unsafe fn disable_raw(self) -> Result<(), DisableError> {
        // Safety: Upheld by the caller
        match unsafe { abi::SOLID_INTC_Disable(abi::c_int(self.0)) } {
            abi::SOLID_ERR_OK => Ok(()),
            abi::SOLID_ERR_PAR => Err(DisableError::BadParam),
            abi::SOLID_ERR_NOTUSED => Err(DisableError::NoHandler),
            abi::c_int(e) => panic!("SOLID_INTC_Disable failed: {e}"),
        }
    }
}

/// The number of interrupt lines. Each SPI counts as one, and each PPI counts
/// one per processor.
const NUM_LINES: usize = abi::GIC_MAXINTNO + (abi::SOLID_CORE_MAX - 1) * 32;

const LINE_TABLE_LEN: usize = (NUM_LINES + usize::BITS as usize - 1) / usize::BITS as usize;
const ZERO: AtomicUsize = AtomicUsize::new(0);
static LINE_SPINLOCK: [AtomicUsize; LINE_TABLE_LEN] = [ZERO; LINE_TABLE_LEN];

/// An interrupt line ID.
///
/// Interrupt line IDs identify the slots filled by `SOLID_INTC_Register`. For
/// PPI, there is a unique set for each processor, which is why the current
/// processor is taken into account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line(usize);

impl Line {
    /// Get the interupt line ID for the specified interrupt number and the current
    /// processor.
    fn from_intno_for_current_processor(Number(intno): Number) -> Option<Self> {
        let intno: usize = intno.try_into().ok()?;
        let processor = if intno < 32 {
            smp::current_processor_id()
        } else {
            abi::SOLID_CORE_MAX - 1
        };
        intno
            .checked_add(processor * 32)
            .filter(|x| *x < NUM_LINES)
            .map(Line)
    }

    fn index_mask(self) -> (usize, usize) {
        let index = self.0 / usize::BITS as usize;
        let mask = 1usize.rotate_left(self.0 as u32);
        (index, mask)
    }

    #[inline]
    fn lock(self) -> impl Sized {
        struct Guard(&'static AtomicUsize, usize);
        impl Drop for Guard {
            #[inline]
            fn drop(&mut self) {
                self.0.fetch_and(self.1, Ordering::Release);
            }
        }

        let (index, mask) = self.index_mask();
        let spinlock = &LINE_SPINLOCK[index];
        while (spinlock.fetch_or(mask, Ordering::Acquire) & mask) != 0 {}

        Guard(spinlock, !mask)
    }
}

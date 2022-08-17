//! High-level binding for SOLID Timer API
//!
//! # External Code Assumptions
//!
//! - All timer handlers in the application return with interrupts disabled.
use core::{
    fmt,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use std::{
    cell::UnsafeCell,
    ptr::{null_mut, NonNull},
};

use crate::{abi, exceptions, interrupt, thread::CpuCx, utils::abort_on_unwind};

/// A SOLID-OS tick count.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TickCount(pub u64);

impl TickCount {
    #[inline]
    pub fn now() -> Self {
        Self(unsafe { abi::SOLID_TIMER_GetCurrentTick() })
    }

    /// Convert `self` to microseconds.
    #[inline]
    pub fn to_usecs(self) -> u64 {
        unsafe { abi::SOLID_TIMER_ToUsec(self.0) }
    }
}

/// Delay the execution for the specified duration by a busy loop.
#[inline]
pub fn sleep_busy_ns(nsecs: Nsecs32) {
    unsafe { abi::SOLID_TIMER_WaitNsec(nsecs.0) }
}

/// An unsigned 32-bit integer value quantifying a length of time in
/// microseconds.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Usecs32(pub u32);

impl From<Usecs32> for Duration {
    #[inline]
    fn from(x: Usecs32) -> Self {
        Duration::from_micros(x.0.into())
    }
}

impl fmt::Debug for Usecs32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Duration::from(*self).fmt(f)
    }
}

/// An unsigned 32-bit integer value quantifying a length of time in
/// nanoseconds.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nsecs32(pub u32);

impl From<Nsecs32> for Duration {
    #[inline]
    fn from(x: Nsecs32) -> Self {
        Duration::from_nanos(x.0.into())
    }
}

impl fmt::Debug for Nsecs32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Duration::from(*self).fmt(f)
    }
}

/// A timer handler for [`Timer`].
///
/// This trait is sealed; it can not be implemented externally.
///
/// `Send`: The handler will run with no synchronization with the creator
/// thread.
///
/// `'static`: An active timer will remain active even if all references to
/// the [`Timer`] are removed. (This is allowed by the pinning guarantees.)
///
/// # Processor states
///
/// The handler is called in an interrupt context
/// ([`solid::exceptions::active`]`() == true`) with interrupts enabled.
///
/// [`solid::exceptions::active`]: crate::exceptions::active
pub trait TimerHandler: Send + private::Sealed + 'static {
    /// Call the timer handler.
    ///
    /// # Safety
    ///
    /// This method can only be called from a SOLID timer handler. It's in
    /// general unsafe to call from user code.
    unsafe fn call(self: Pin<&mut Self>, cx: CpuCx<'_>);
}

/// The implementation of `TimerHandler` for closures.
///
/// `Unpin`: `FnMut` requires an unpinned receiver.
impl<T: FnMut(CpuCx<'_>) + Unpin + Send + 'static> TimerHandler for T {
    #[inline]
    unsafe fn call(mut self: Pin<&mut Self>, cx: CpuCx<'_>) {
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                interrupt::disable();
            }
        }

        // Re-enable interrupts.
        //
        // Safety: We are not inside a SOLID critical section. We make sure to
        // re-disable interrupts before returning. See "External Code
        // Assumptions".
        unsafe { interrupt::enable() };
        let _guard = Guard;

        (*self)(cx);
    }
}

impl<T: TimerHandler> TimerHandler for Option<T> {
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
    /// `TimerHandler`)
    pub trait Sealed {}
    impl<T: FnMut(CpuCx<'_>) + Unpin + Send + 'static> Sealed for T {}
    impl<T: TimerHandler> Sealed for Option<T> {}
}

/// The schedule for [`Timer`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(i32)]
pub enum Schedule {
    /// Call the handler once after the specified period of time.
    OneShot(Usecs32),
    /// Call the handler periodically with the specified interval.
    Interval(Usecs32),
    /// Call the handler once when the specified point of time is reached.
    GlobalTick(TickCount),
}

impl Schedule {
    /// Update fields of `SOLID_TIMER_HANDLER` according to `self`.
    #[inline]
    const fn update_sys(&self, ty: &mut u32, time: &mut u32, global_tick: &mut u64) {
        match self {
            &Schedule::OneShot(Usecs32(x)) => {
                *ty = abi::SOLID_TIMER_TYPE_ONESHOT;
                *time = x;
            }
            &Schedule::Interval(Usecs32(x)) => {
                *ty = abi::SOLID_TIMER_TYPE_INTERVAL;
                *time = x;
            }
            &Schedule::GlobalTick(TickCount(x)) => {
                *ty = abi::SOLID_TIMER_TYPE_GLOBALTICK;
                *global_tick = x;
            }
        }
    }
}

#[inline]
fn is_oneshot_ty(timer_ty: u32) -> bool {
    timer_ty != abi::SOLID_TIMER_TYPE_INTERVAL
}

/// The error type for [`Timer::start`].
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TimerStartError {
    /// The current processor may not use the timer API safely.
    ///
    /// This error will be returned under the following cases:
    ///
    /// - `SOLID_REQUIRES_GLOBALTICK` is not defined, and the current processor
    ///   is not the first processor (CPU 0) in the system.
    ///
    BadProcessor,
    /// The parameters of the timer are incorrect. For example, the timer
    /// interval ([`Schedule::Interval`]) is set to zero.
    BadParam,
}

/// The error type for [`Timer::stop`].
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TimerStopError {
    /// The timer was started by a different processor and is still running.
    BadProcessor,
    /// The method was called from an interrupt context, and the timer is
    /// running.
    ///
    /// We decided not to support calling this method from an interrupt context
    /// because a call to the system timer handler might be in progress in this
    /// case, and the corresponding `SOLID_TIMER_HANDLER` might already be
    /// included in the system timer handler's local execution queue, from which
    /// we can't remove `SOLID_TIMER_HANDLER`, hence `Timer::stop` is unable to
    /// guarantee the cessation of timer handler calls and the system ownership
    /// of `SOLID_TIMER_HANDLER`.
    BadContext,
}

/// An error type indicating a [`Timer`] is still running.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RunningError;

/// The safe wrapper for a SOLID-OS timer.
pub struct Timer<T: TimerHandler> {
    inner: UnsafeCell<abi::SOLID_TIMER_HANDLER>,
    /// [`STOPPED`] if the timer is stopped, or `processor_id + 1` otherwise.
    owning_processor_id_p1: AtomicUsize,
    handler: UnsafeCell<T>,
    /// `SOLID_TIMER_HANDLER` must remain in place as long as the timer is
    /// active.
    _pin: core::marker::PhantomPinned,
}

const STOPPED: usize = 0;

/// `Timer` can be controlled by any threads, though the operations may fail at
/// runtime if the preconditions are not satisfied.
unsafe impl<T: TimerHandler> Send for Timer<T> {}
/// `&Timer` permits no operations, hence this is safe.
unsafe impl<T: TimerHandler> Sync for Timer<T> {}

/// The destructor for [`Timer`].
///
/// # Panics
///
/// **`drop` will abort if it fails to stop the timer by [`Self::stop`].** See
/// [`TimerStopError`] for the possibly failure causes.
impl<T: TimerHandler> Drop for Timer<T> {
    fn drop(&mut self) {
        // We must absolutely make sure the timer is stopped (i.e., the
        // system ownership of `SOLID_TIMER_HANDLER` is relinquished) before
        // returning.
        abort_on_unwind(|| {
            // Safety: `drop` can always do this safely.
            let this = unsafe { Pin::new_unchecked(self) };
            this.stop()
                .expect("failed to stop timer in prior to disposing it");
        })
    }
}

impl<T: TimerHandler> Timer<T> {
    /// Construct a stopped `Timer`.
    #[inline]
    pub const fn new(schedule: Schedule, handler: T) -> Self {
        let mut inner = abi::SOLID_TIMER_HANDLER {
            pNext: null_mut(),
            pCallQ: null_mut(),
            globalTick: 0,
            ty: 0,
            time: 0,
            func: Self::handler_trampoline as _,
            param: null_mut(), // set later
        };
        schedule.update_sys(&mut inner.ty, &mut inner.time, &mut inner.globalTick);
        Self {
            inner: UnsafeCell::new(inner),
            owning_processor_id_p1: AtomicUsize::new(STOPPED),
            handler: UnsafeCell::new(handler),
            _pin: core::marker::PhantomPinned,
        }
    }

    /// The outer timer handler.
    unsafe extern "C" fn handler_trampoline(param: *mut u8, cpu_cx: *mut abi::SOLID_CPU_CONTEXT) {
        abort_on_unwind(|| {
            debug_assert!(exceptions::active());

            // Safety: `param`'s value is taken from the corresponding timer's
            // `SOLID_TIMER_HANDLER::param`. `Self::start` derives this value by
            // casting `&Timer`. Since `start` takes a pinned receiver, user
            // code can not invalidate `*this` without our code intervening. Our
            // `Self::drop` makes sure that all handler calls are complete
            // before the storage of `*this` is reclaimed for other uses.
            let this = unsafe { &*param.cast::<Self>() };

            let cpu_cx = NonNull::new(cpu_cx).expect("null cpu context");
            let cpu_cx = CpuCx::new(cpu_cx);

            // Safety: While the timer is running, we the handler have the
            // exclusive ownership of `T`
            let handler = unsafe { &mut *this.handler.get() };
            // Safety: `Self` is `!Unpin`, and we maintain the pinning
            // guarantees of `this.handler`
            let handler = unsafe { Pin::new_unchecked(handler) };
            // Safety: We are calling it from a timer handler, which is allowed
            // to do this
            unsafe { handler.call(cpu_cx) };

            // Get the type of this timer.
            // Safety: While the timer is running, this field is immutable
            let ty = unsafe { (*this.inner.get()).ty };

            // If it's a oneshot timer, update our internal structure to
            // indicate that the timer is now stopped. Since this transfers the
            // ownership of everything inside `*this`, to whoever owns
            // `Pin<&mut Timer>` now, we have to use `Release` ordering.
            debug_assert_ne!(this.owning_processor_id_p1.load(Ordering::Relaxed), STOPPED);
            if is_oneshot_ty(ty) {
                this.owning_processor_id_p1
                    .store(STOPPED, Ordering::Release);
            }
        })
    }

    /// Check if the timer is currently running.
    ///
    /// # Memory Ordering
    ///
    /// If this method returns `false`, all preceding timer handler invocations
    /// synchronize-with the call.
    #[inline]
    pub fn is_running(&self) -> bool {
        self.owning_processor_id_p1.load(Ordering::Acquire) != STOPPED
    }

    /// Start the timer. Returns `true` if the timer was previously inactive
    /// and is now active.
    pub fn start(self: Pin<&mut Self>) -> Result<bool, TimerStartError> {
        if self.is_running() {
            return Ok(false);
        }

        // Disable interrupts to prevent processor migration
        interrupt::free(|_| {
            // Safety: `self.is_running() == false` combined with the possession
            // of `&mut Self` means we have an exclusive ownership of everything
            // inside `*self`.
            let inner = unsafe { &mut *self.inner.get() };

            // If `defined(SOLID_TIMER_EACHCPU)`, there's a `Timer` for each
            // processor. `SOLID_TIMER_[Un]RegisterTimer` uses the current
            // processor's instance.
            //
            // Otherwise, there's a single global `Timer` instance having an
            // interrupt handler assigned to the first processor. It's unsafe
            // to call `SOLID_TIMER_[Un]RegisterTimer` from other processors in
            // this case.
            let current_processor_id_p1 = current_processor_id_p1();
            if !abi::SOLID_TIMER_EACHCPU && current_processor_id_p1 != 1 {
                return Err(TimerStartError::BadProcessor);
            }

            // Assign `SOLID_TIMER_HANDLER::param`. Since `*self` is pinned, we
            // know its address is stable until `Self::drop` is called.
            inner.param = &*self as *const _ as *mut _;

            // Add `self.inner` to the system timer queue (`Timer::m_pQueue`).
            //
            // Safety:
            // - The current processor is the same as the one where
            //   `Timer::Notify` calls this timer handler, so the surrounding
            //   `interrupt::free` block protects `m_pQueue` from being
            //   simultanouesly accessed by `Timer::Notify`.
            // - `self.inner` is not registered.
            // - `inner.param` is initialized properly (the precondition of
            //   `Self::handler_trampoline`).
            // - External code assumption: All timer handlers return with
            //   interrupts disabled, so we are not running inside an interrupt
            //   handler that started by preempting the execution of the part of
            //   `Timer::Notify` that accesses `Timer::m_pQueue`.
            let result = unsafe { abi::SOLID_TIMER_RegisterTimer(self.inner.get()) };
            match result {
                abi::SOLID_ERR_PAR => return Err(TimerStartError::BadParam),
                abi::SOLID_ERR_OK => {}
                abi::c_int(e) => panic!("SOLID_TIMER_UnRegisterTimer failed: {e}"),
            }

            // Remember the processor associated with the timer. This write
            // must happen-before the corresponding `stop` call and the store
            // done by the corresponding `handler_trampoline` call (if it's a
            // oneshot timer). The former is fulfilled by virtue of taking a
            // `Pin<&mut Self>` receiver. The latter is satisfied by doing this
            // in `interrupt::free`.
            self.owning_processor_id_p1
                .store(current_processor_id_p1, Ordering::Relaxed);

            Ok(true)
        })
    }

    /// Stop the timer. Returns `true` if the timer was previously active and
    /// is now inactive.
    ///
    /// The successful completion of a call to this method ensures the
    /// completion of all previous and ongoing handler invocations.
    /// All handler invocations that occurred while the timer was active will
    /// synchronize-with the call.
    pub fn stop(self: Pin<&mut Self>) -> Result<bool, TimerStopError> {
        // Get the processor on which the timer is active. If it's inactive,
        // synchronize-with all preceding timer handler calls.
        let owning_processor_id_p1 = self.owning_processor_id_p1.load(Ordering::Acquire);
        if owning_processor_id_p1 == STOPPED {
            return Ok(false);
        }

        // If we are in an interrupt context, there are the following
        // possibilities:
        //
        // - We are in the system timer handler, in which case `self.inner` may
        //   already be included in the handler's local execution queue, so
        //   `SOLID_TIMER_UnRegisterTimer` might not immediately remove the
        //   system ownership of `self.inner`. There's no way to tell whether
        //   `self.inner` has already been released by the system timer handler
        //   or not.  This means we wouldn't be able to reset
        //   `self.owning_processor_id_p1` at the correct timing if we did
        //   `SOLID_TIMER_UnRegisterTimer` in this case.
        //
        // - We are inside an interrupt handler started by preempting the
        //   execution of a timer handler (but not in an arbitrary point of
        //   `Notify::Timer`; see "External Code Assumptions"). `SOLID_TIMER_
        //   [Un]RegisterTimer` internally employs a critical section, so there
        //   is no danger of data races. However, since we are essentially
        //   executing in the system timer handler, the previous bullet point
        //   applies to this case.
        if exceptions::active() {
            return Err(TimerStopError::BadContext);
        }

        // Disable interrupts to prevent processor migration and exclude the
        // system timer handler (`Timer::Notify`) execution (i.e., all preceding
        // `Timer::Notify` calls synchronize-with this block, and this block
        // synchronizes-with all upcoming ones).
        interrupt::free(|_| {
            let current_processor_id_p1 = current_processor_id_p1();
            if owning_processor_id_p1 != current_processor_id_p1 {
                return Err(TimerStopError::BadProcessor);
            }

            // Remove `self.inner` from the system timer queue
            // (`Timer::m_pQueue`).
            //
            // Safety:
            // - The current processor is the same as the one that registered
            //   the timer, so this will operate on the correct `Timer`
            //   instance.
            // - The current processor is the same as the one where
            //   `Timer::Notify` calls this timer handler, so the surrounding
            //   `interrupt::free` block protects `m_pQueue` from being
            //   simultanouesly accessed by `Timer::Notify`.
            let result = unsafe { abi::SOLID_TIMER_UnRegisterTimer(self.inner.get()) };

            match result {
                abi::SOLID_ERR_OK => {}
                abi::SOLID_ERR_NOTFOUND => {
                    // This oneshot timer must have fired just before
                    // `interrupt::free`.
                }
                abi::c_int(e) => panic!("SOLID_TIMER_UnRegisterTimer failed: {e}"),
            }

            // `Relaxed` ordering suffices because we are now the exclusive
            // owner of everything inside `*self`.
            self.owning_processor_id_p1
                .store(STOPPED, Ordering::Relaxed);

            Ok(result == abi::SOLID_ERR_OK)
        })
    }

    /// Set a new schedule for a stopped timer.
    #[inline]
    pub fn reschedule(self: Pin<&mut Self>, schedule: Schedule) -> Result<(), RunningError> {
        if self.is_running() {
            Err(RunningError)
        } else {
            // Safety: `self.is_running() == false` combined with the possession
            // of `&mut Self` means we have an exclusive ownership of everything
            // inside `*self`.
            let inner = unsafe { &mut *self.inner.get() };
            schedule.update_sys(&mut inner.ty, &mut inner.time, &mut inner.globalTick);
            Ok(())
        }
    }

    /// Borrow the `T` (timer handler) of a stopped timer.
    #[inline]
    pub fn handler_pin(self: Pin<&Self>) -> Result<Pin<&T>, RunningError> {
        if self.is_running() {
            Err(RunningError)
        } else {
            // Safety: `self.is_running() == false` combined with the possession
            // of `&Self` means we have a shared ownership of everything
            // inside `*self`.
            Ok(unsafe { Pin::new_unchecked(&*self.handler.get()) })
        }
    }

    /// Mutably borrow the `T` (timer handler) of a stopped timer.
    #[inline]
    pub fn handler_pin_mut(self: Pin<&mut Self>) -> Result<Pin<&mut T>, RunningError> {
        if self.is_running() {
            Err(RunningError)
        } else {
            // Safety: `self.is_running() == false` combined with the possession
            // of `&mut Self` means we have an exclusive ownership of everything
            // inside `*self`.
            Ok(unsafe { Pin::new_unchecked(&mut *self.handler.get()) })
        }
    }
}

#[inline]
fn current_processor_id_p1() -> usize {
    crate::smp::current_processor_id() + 1
}

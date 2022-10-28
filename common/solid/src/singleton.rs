//! Provides a macro [`singleton!`] to create a static storage that can be
//! mutably borrowed only once.
use core::{fmt, mem::MaybeUninit, pin::Pin};
use takecell::TakeCell;

/// Macro to create a pinned mutable reference to a statically-allocated value.
///
/// This macro returns a value of type `Result<Pin<&'static mut $ty>,
/// `[`AlreadyBorrowedError`]`>`.
/// `Ok(_)` will be returned the first time the macro expression is evaluated;
/// further calls will return `Err(_)`. To avoid `unwrap`ping a `None` variant,
/// the caller must ensure that the expression is evaluated by a function that
/// is executed at most once in the whole lifetime of a program.
///
/// This macro was designed after [`cortex_m::singleton!`][1].
///
/// # Type inference
///
/// Specific types provided by this crate, such as [`Timer`][2] and
/// [`Handler`][3], are special-cased so that they can have their type parameter
/// elided as in `Timer<_>`. This feature requires enabling
/// `#![feature(type_alias_impl_trait)]` ([rust-lang/rust#63063][4]) in your
/// application crate.
///
/// # Example
///
/// ```rust,no_run
/// use solid::{singleton::pin_singleton, timer::{Schedule, Timer, Usecs32}};
///
/// let mut timer = pin_singleton!(
///     : Timer<_> = Timer::new(
///         Schedule::Interval(Usecs32(200_000)),
///         |_: solid::thread::CpuCx<'_>| {
///             // ...
///         },
///     )
/// )
/// .unwrap();
///
/// // note: Pin::as_mut is used here to reborrow from `timer` instead of
/// // consuming it
/// timer.as_mut().start().expect("unable to start timer");
///
/// assert!(timer.is_running());
/// ```
///
/// [1]: https://docs.rs/cortex-m/0.7.6/cortex_m/macro.singleton.html
/// [2]: crate::timer::Timer
/// [3]: crate::interrupt::Handler
/// [4]: https://github.com/rust-lang/rust/issues/63063
pub macro pin_singleton {
    ($($name:ident)?: Timer<_> = $expr:expr $(,)?) => {{
        type TimerHandlerTy = impl $crate::timer::TimerHandler;
        pin_singleton!(: $crate::timer::Timer<TimerHandlerTy> = $expr)
    }},
    ($($name:ident)?: Handler<_> = $expr:expr $(,)?) => {{
        type HandlerFnTy = impl $crate::interrupt::HandlerFn;
        pin_singleton!(: $crate::interrupt::Handler<HandlerFnTy> = $expr)
    }},
    ($name:ident: $ty:ty = $expr:expr $(,)?) => {
        {
            static $name: $crate::singleton::LazyPinTakeCell<$ty> =
                $crate::singleton::LazyPinTakeCell::new();
            &$name
        }.take(|| $expr)
    },
    (: $ty:ty = $expr:expr $(,)?) => {
        pin_singleton!(VAR: $ty = $expr)
    },
}

/// An error type indicating that a static storage defined by [`singleton!`]
/// was mutably borrowed once and can not be borrowed again.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AlreadyBorrowedError;

impl fmt::Display for AlreadyBorrowedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("already borrowed")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AlreadyBorrowedError {}

#[doc(hidden)]
pub struct LazyPinTakeCell<T> {
    inner: TakeCell<MaybeUninit<T>>,
}

impl<T> LazyPinTakeCell<T> {
    pub const fn new() -> Self {
        Self {
            inner: TakeCell::new(MaybeUninit::uninit())
        }
    }
    
    #[inline]
    pub fn take(&'static self, init: impl FnOnce() -> T) -> Result<Pin<&'static mut T>,  AlreadyBorrowedError> {
        let storage = self.inner.take().ok_or(AlreadyBorrowedError)?;
        Ok(Pin::static_mut(storage.write(init())))
    }
}

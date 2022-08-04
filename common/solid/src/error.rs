//! Error types
use core::{fmt, num::NonZeroI32};

/// An unprocessed SOLID error code.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Error {
    // TODO: Restrict to a negative value
    code: NonZeroI32,
}

impl Error {
    /// Construct `Self` from a raw error code.
    #[inline]
    pub const fn from_raw(code: i32) -> Option<Self> {
        if let Some(code) = NonZeroI32::new(code) {
            if code.get() < 0 {
                Some(Self { code })
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(&self) -> i32 {
        self.code.get()
    }

    #[inline]
    pub(crate) fn err_if_negative(x: i32) -> Result<i32, Self> {
        if let Some(x) = Error::from_raw(x) {
            Err(x)
        } else {
            Ok(x)
        }
    }
}

#[macropol::macropol]
macro_rules! define_error_codes {
    ($($ident:ident),*$(,)*) => {
        $(paste::paste! {
            /// `SOLID_ERR_$&ident`
            pub const $ident: Self = {
                if let Some(x) = Self::from_raw(crate::abi::[< SOLID_ERR_ $ident >].0) {
                    x
                } else {
                    unreachable!();
                }
            };
        })*
    };
}

/// Known error codes
impl Error {
    define_error_codes!(
        PAR,
        MACV,
        NOMEM,
        NORES,
        NOTFOUND,
        NOTSUPPORTED,
        EBADF,
        INVALIDCONTENT,
        NOTUSED,
        ALREADYUSED,
        OUTOFBOUND,
        BADSEQUENCE,
        UNKNOWNDEVICE,
        BUSY,
        TIMEOUT,
        INVALIDACCESS,
        NOTREADY,
    );
}

impl From<Error> for std::io::Error {
    #[inline]
    fn from(e: Error) -> Self {
        std::io::Error::from_raw_os_error(e.get())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::io::Error::from(*self).fmt(f)
    }
}

impl std::error::Error for Error {}

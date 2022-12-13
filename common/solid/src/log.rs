//! Logging
use core::fmt;

use crate::abi;

/// Write a single byte to the current log target.
#[inline]
pub fn write_byte(b: u8) {
    unsafe { abi::SOLID_LOG_PutChar(b as i8) };
}

/// Write bytes to the current log target.
#[inline]
pub fn write_bytes(b: &[u8]) {
    unsafe { abi::SOLID_LOG_write(b.as_ptr().cast(), b.len()) };
}

/// Write a string to the current log target.
#[inline]
pub fn write_str(b: &str) {
    write_bytes(b.as_bytes());
}

/// Write a formatted string to the current log target.
pub fn write_fmt(args: fmt::Arguments<'_>) {
    let _ = fmt::write(&mut Writer, args);
}

pub macro write($($arg:tt)*) {
    $crate::log::write_fmt($crate::core::format_args!($($arg)*))
}

pub macro writeln($($arg:tt)*) {
    {
        $crate::log::write_fmt($crate::core::format_args!($($arg)*));
        $crate::log::write_byte(b'\n');
    }
}

/// Implementation of [`fmt::Write`] that forwards written bytes to
/// [`write_bytes`].
#[derive(Clone, Copy, Debug)]
pub struct Writer;

impl fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_bytes(s.as_bytes());
        Ok(())
    }
}

impl std::io::Write for Writer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        write_bytes(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

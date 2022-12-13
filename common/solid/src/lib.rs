#![deny(unsafe_op_in_unsafe_fn)]
#![feature(const_mut_refs)]
#![feature(generic_associated_types)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset_from)]
#![feature(const_size_of_val)]
#![feature(decl_macro)]

#[cfg(not(feature = "std"))]
compile_error!("feature `std` is currently required due to `autocxx`'s requirements");

#[doc(hidden)]
pub extern crate core;

pub mod abi;
pub mod closure;
pub mod error;
pub mod exceptions;
pub mod fs;
pub mod interrupt;
pub mod loader;
pub mod log;
pub mod singleton;
pub mod smp;
#[doc(hidden)]
pub mod staticenv;
pub mod thread;
pub mod timer;
mod utils;

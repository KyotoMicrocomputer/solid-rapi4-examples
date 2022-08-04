#![deny(unsafe_op_in_unsafe_fn)]
#![feature(const_mut_refs)]

#[cfg(not(feature = "std"))]
compile_error!("feature `std` is currently required due to `autocxx`'s requirements");

pub mod abi;
pub mod error;
pub mod interrupt;
pub mod smp;
pub mod thread;
pub mod timer;
mod utils;

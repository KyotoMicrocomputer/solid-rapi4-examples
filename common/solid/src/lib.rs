#![deny(unsafe_op_in_unsafe_fn)]
#![feature(const_mut_refs)]
#![feature(generic_associated_types)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset_from)]

#[cfg(not(feature = "std"))]
compile_error!("feature `std` is currently required due to `autocxx`'s requirements");

pub mod abi;
pub mod error;
pub mod fs;
pub mod interrupt;
pub mod loader;
pub mod smp;
pub mod thread;
pub mod timer;
mod utils;

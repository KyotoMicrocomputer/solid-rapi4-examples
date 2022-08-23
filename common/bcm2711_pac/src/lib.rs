#![doc = include_str!("../README.md")]
#![no_std]
mod bus;
mod field;
pub use {bus::*, field::*};

pub mod ap804;
// `aux.rs` breaks some tools on Windows
// https://msdn.microsoft.com/en-us/library/aa365247(v=vs.85).aspx#file_and_directory_names
#[path = "aux_.rs"]
pub mod aux;
pub mod bsc;
pub mod dmac;
pub mod gpio;
pub mod mbox;
pub mod pcm;
pub mod pl011;
pub mod pwm;
pub mod spi;
pub mod sys_timer;

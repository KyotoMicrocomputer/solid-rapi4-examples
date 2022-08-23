//! [System Timer][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A145%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    fields::{Field, FieldValue},
    register_structs,
    registers::{ReadOnly, ReadWrite},
    RegisterLongName,
};

use crate::Vpa;

/// The base address of [the System Timer peripheral register block](Registers).
pub const BASE: Vpa = Vpa(0x4_7e00_3000);

register_structs! {
    pub Registers {
        /// Control/status
        (0x00 => pub cs: ReadWrite<u32, CS::Register>),
        /// Counter lower 32 bits
        (0x04 => pub clo: ReadOnly<u32>),
        /// Counter higher 32 bits
        (0x08 => pub chi: ReadOnly<u32>),
        /// Compare `0..4`
        (0x0c => pub c: [ReadWrite<u32>; 4]),
        (0x1c => @END),
    }
}

#[allow(non_snake_case)]
pub mod CS {
    use super::*;
    pub struct Register;
    impl RegisterLongName for Register {}

    /// Construct a [`Field`] representing the `M` bit (System Timer Match, W1C)
    /// corresponding to the specified comparator number.
    ///
    /// # Panic
    ///
    /// Panics if `i` is outside the range `0..4`.
    #[inline]
    pub const fn M(i: usize) -> Field<u32, Register> {
        assert!(i < 4);
        Field::new(0b1, i)
    }

    /// Construct a [`FieldValue`] to clear the [`M`] bit corresponding to
    /// the specified comparator number.
    ///
    /// # Panic
    ///
    /// Panics if `i` is outside the range `0..4`.
    #[inline]
    pub const fn M_clear(i: usize) -> FieldValue<u32, Register> {
        FieldValue::<u32, _>::new(0b1, i, 0b1)
    }
}

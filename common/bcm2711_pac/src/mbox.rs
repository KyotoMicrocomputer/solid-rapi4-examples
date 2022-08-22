//! [BCM2711 ARM Mailboxes][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A166%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_structs,
    registers::{ReadWrite, WriteOnly},
};

/// The low-peripheral ARM physical address of [the ARM Mailboxes register
/// block](Registers).
pub const BASE_ARM_PA: u64 = 0xff80_0080;

register_structs! {
    pub Registers {
        /// Set bit register
        (0x00 => pub mbox_set: [WriteOnly<u32>; 16]),
        /// Clear bit register
        (0x40 => pub mbox_clr: [ReadWrite<u32>; 16]),
        (0x80 => @END),
    }
}

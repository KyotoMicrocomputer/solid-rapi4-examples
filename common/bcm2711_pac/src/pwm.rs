//! [BCM2711 PWM][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A130%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

use crate::Vpa;

/// The base address of the PWM0 instance of [the PWM register block](Registers).
pub const BASE_PWM0: Vpa = Vpa(0x4_7e20_c000);
/// The base address of the PWM1 instance of [the PWM register block](Registers).
pub const BASE_PWM1: Vpa = Vpa(0x4_7e20_c800);

register_structs! {
    pub Registers {
        /// PWM control
        (0x00 => pub ctl: ReadWrite<u32, CTL::Register>),
        /// PWM status
        (0x04 => pub sta: ReadWrite<u32, STA::Register>),
        /// PMA DMA configuration
        (0x08 => pub dmac: ReadWrite<u32, DMAC::Register>),
        (0x0c => _pad0),
        /// PWM channel 1 range
        (0x10 => pub rng1: ReadWrite<u32>),
        /// PWM channel 1 data
        (0x14 => pub dat1: ReadWrite<u32>),
        /// PWM FIFO input
        (0x18 => pub fif1: WriteOnly<u32>),
        (0x1c => _pad1),
        /// PWM channel 2 range
        (0x20 => pub rng2: ReadWrite<u32>),
        /// PWM channel 2 data
        (0x24 => pub dat2: ReadWrite<u32>),
        (0x28 => @END),
    }
}

register_bitfields! {u32,
    pub CTL [
        /// Channel 1 enable
        PWEN1 OFFSET(0) NUMBITS(1) [],
        /// Channel 1 mode
        MODE1 OFFSET(1) NUMBITS(1) [
            Pwm = 0,
            Serialiser = 1,
        ],
        /// Channel 1 repeat last data
        RPTL1 OFFSET(2) NUMBITS(1) [],
        /// Channel 1 silence bit
        SBIT1 OFFSET(3) NUMBITS(1) [],
        /// Channel 1 polarity
        POLA1 OFFSET(4) NUMBITS(1) [
            ActiveHigh = 1,
            ActiveLow = 0,
        ],
        /// Channel 1 use FIFO
        USEF1 OFFSET(5) NUMBITS(1) [],
        /// Clear FIFO
        CLRF OFFSET(6) NUMBITS(1) [],
        /// Channel 1 M/S enable
        MSEN1 OFFSET(7) NUMBITS(1) [
            Pwm = 0,
            Ms = 1,
        ],
        /// Channel 2 enable
        PWEN2 OFFSET(8) NUMBITS(1) [],
        /// Channel 2 mode
        MODE2 OFFSET(9) NUMBITS(1) [
            Pwm = 0,
            Serialiser = 1,
        ],
        /// Channel 1 repeat last data
        RPTL2 OFFSET(10) NUMBITS(1) [],
        /// Channel 2 silence bit
        SBIT2 OFFSET(11) NUMBITS(1) [],
        /// Channel 2 polarity
        POLA2 OFFSET(12) NUMBITS(1) [
            ActiveHigh = 1,
            ActiveLow = 0,
        ],
        /// Channel 2 use FIFO
        USEF2 OFFSET(13) NUMBITS(1) [],
        /// Channel 2 M/S enable
        MSEN2 OFFSET(15) NUMBITS(1) [
            Pwm = 0,
            Ms = 1,
        ],
    ]
}

register_bitfields! {u32,
    pub STA [
        /// FIFO full flag (RO)
        FULL1 OFFSET(0) NUMBITS(1) [],
        /// FIFO empty flag (RO)
        EMPT1 OFFSET(1) NUMBITS(1) [],
        /// FIFO write error flag (W1C)
        WERR1 OFFSET(2) NUMBITS(1) [],
        /// FIFO read error flag (W1C)
        RERR1 OFFSET(3) NUMBITS(1) [],
        /// Channel 1 gap occurred flag (W1C)
        GAPO1 OFFSET(4) NUMBITS(1) [],
        /// Channel 2 gap occurred flag (W1C)
        GAPO2 OFFSET(5) NUMBITS(1) [],
        /// Bus error flag (W1C)
        BERR OFFSET(8) NUMBITS(1) [],
        /// Channel 1 state (RO)
        STA1 OFFSET(9) NUMBITS(1) [],
        /// Channel 2 state (RO)
        STA2 OFFSET(10) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub DMAC [
        /// DMA threshold for DREQ signal
        DREQ OFFSET(0) NUMBITS(8) [],
        /// DMA threshold for PANIC signal
        PANIC OFFSET(8) NUMBITS(8) [],
        /// DMA enable
        ENAB OFFSET(31) NUMBITS(1) [],
    ]
}

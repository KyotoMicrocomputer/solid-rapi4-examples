//! [Broadcom Serial Control controllers][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A27%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

use crate::Vpa;

/// The base address of the BSC0 instance of [the BSC register block](Registers).
pub const BASE_BSC0: Vpa = Vpa(0x4_7e20_5000);
/// The base address of the BSC1 instance of [the BSC register block](Registers).
pub const BASE_BSC1: Vpa = Vpa(0x4_7e80_4000);
/// The base address of the BSC3 instance of [the BSC register block](Registers).
pub const BASE_BSC3: Vpa = Vpa(0x4_7e20_5600);
/// The base address of the BSC4 instance of [the BSC register block](Registers).
pub const BASE_BSC4: Vpa = Vpa(0x4_7e20_5800);
/// The base address of the BSC5 instance of [the BSC register block](Registers).
pub const BASE_BSC5: Vpa = Vpa(0x4_7e20_5a80);
/// The base address of the BSC6 instance of [the BSC register block](Registers).
pub const BASE_BSC6: Vpa = Vpa(0x4_7e20_5c00);

register_structs! {
    pub Registers {
        /// Control
        (0x00 => pub c: ReadWrite<u32, C::Register>),
        /// Status
        (0x04 => pub s: ReadWrite<u32, S::Register>),
        /// Data length
        (0x08 => pub dlen: ReadWrite<u32, DLEN::Register>),
        /// Slave address
        (0x0c => pub a: ReadWrite<u32, A::Register>),
        /// Data FIFO
        (0x10 => pub fifo: ReadWrite<u32, FIFO::Register>),
        /// Clock divider
        (0x14 => pub div: ReadWrite<u32, DIV::Register>),
        /// Data delay
        (0x18 => pub del: ReadWrite<u32, DEL::Register>),
        /// Clock stretch timeout
        (0x1c => pub clkt: ReadWrite<u32, CLKT::Register>),
        (0x20 => @END),
    }
}

register_bitfields! {u32,
    pub C [
        /// Read transfer
        READ OFFSET(0) NUMBITS(1) [
            Write = 0,
            Read = 1,
        ],
        /// FIFO clear (W1SC)
        CLEAR OFFSET(4) NUMBITS(2) [
            NoAction = 0b00,
            Clear = 0b01, // `x1` and `1x` both will work
        ],
        /// Start transfer (W1SC)
        ST OFFSET(7) NUMBITS(1) [
            NoAction = 0,
            StartNewTransfer = 1,
        ],
        /// Interrupt on DONE
        INTD OFFSET(8) NUMBITS(1) [],
        /// Interrupt on TX
        INTT OFFSET(9) NUMBITS(1) [],
        /// Interrupt on RX
        INTR OFFSET(10) NUMBITS(1) [],
        /// I2C enable
        I2CEN OFFSET(15) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub S [
        /// Transfer active (RO)
        TA OFFSET(0) NUMBITS(1) [],
        /// Transfer done (W1C)
        DONE OFFSET(1) NUMBITS(1) [],
        /// FIFO needs writing (RO)
        TXW OFFSET(2) NUMBITS(1) [],
        /// FIFO needs reading (RO)
        RXR OFFSET(3) NUMBITS(1) [],
        /// FIFO can accept data (RO)
        TXD OFFSET(4) NUMBITS(1) [],
        /// FIFO contains data (RO)
        RXD OFFSET(5) NUMBITS(1) [],
        /// FIFO empty (RO)
        TXE OFFSET(6) NUMBITS(1) [],
        /// FIFO full (RO)
        RXF OFFSET(7) NUMBITS(1) [],
        /// ACK error (W1C)
        ERR OFFSET(8) NUMBITS(1) [],
        /// Clock stretch timeout (W1C)
        CLKT OFFSET(9) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub DLEN [
        /// Data length in bytes
        DLEN OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub A [
        /// Slave address
        ADDR OFFSET(0) NUMBITS(7) [],
    ]
}

register_bitfields! {u32,
    pub FIFO [
        DATA OFFSET(0) NUMBITS(8) [],
    ]
}

register_bitfields! {u32,
    pub DIV [
        /// Clock divider
        CDIV OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub DEL [
        /// Rising edge delay
        REDL OFFSET(0) NUMBITS(16) [],
        /// Falling edge delay
        FEDL OFFSET(16) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub CLKT [
        /// Clock stretch timeout value
        TOUT OFFSET(0) NUMBITS(16) [],
    ]
}

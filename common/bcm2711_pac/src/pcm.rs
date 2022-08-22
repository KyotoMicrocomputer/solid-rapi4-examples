//! [BCM2711 PCM / I2S Audio][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A115%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

use crate::Vpa;

/// The base address of [the PCM register block](Registers).
pub const BASE: Vpa = Vpa(0x4_7e20_3000);

register_structs! {
    pub Registers {
        (0x00 => pub cs_a: ReadWrite<u32, CS_A::Register>),
        (0x04 => pub fifo_a: ReadWrite<u32, FIFO_A::Register>),
        (0x08 => pub mode_a: ReadWrite<u32, MODE_A::Register>),
        (0x0c => pub rxc_a: ReadWrite<u32, RXC_A::Register>),
        (0x10 => pub txc_a: ReadWrite<u32, TXC_A::Register>),
        (0x14 => pub dreq_a: ReadWrite<u32, DREQ_A::Register>),
        (0x18 => pub inten_a: ReadWrite<u32, INTEN_A::Register>),
        (0x1c => pub intstc_a: ReadWrite<u32, INTSTC_A::Register>),
        (0x20 => pub gray: ReadWrite<u32, GRAY::Register>),
        (0x24 => @END),
    }
}

register_bitfields! {u32,
    pub CS_A [
        /// Enable the PCM audio interface
        EN OFFSET(0) NUMBITS(1) [],
        /// Enable reception
        RXON OFFSET(1) NUMBITS(1) [],
        /// Enable transmission
        TXON OFFSET(2) NUMBITS(1) [],
        /// Clear the TX FIFO
        TXCLR OFFSET(3) NUMBITS(1) [],
        /// Clear the RX FIFO
        RXCLR OFFSET(4) NUMBITS(1) [],
        /// Set the TX FIFO threshold at which point the `TXW` flag is set
        TXTHR OFFSET(5) NUMBITS(2) [
            Empty = 0b00,
            OneQuarterFull = 0b01,
            ThreeQuartersFull = 0b10,
            FullButOneSample = 0b11,
        ],
        /// Set the RX FIFO threshold at which point the `RXR` flag is set
        RXTHR OFFSET(7) NUMBITS(2) [
            OneSample = 0b00,
            OneQuarterFull = 0b01,
            ThreeQuartersFull = 0b10,
            Full = 0b11,
        ],
        /// DMA DREQ enable
        DMAEN OFFSET(9) NUMBITS(1) [],
        /// TX FIFO sync
        TXSYNC OFFSET(13) NUMBITS(1) [],
        /// RX FIFO sync
        RXSYNC OFFSET(14) NUMBITS(1) [],
        /// TX FIFO error
        TXERR OFFSET(15) NUMBITS(1) [],
        /// RX FIFO error
        RXERR OFFSET(16) NUMBITS(1) [],
        /// Indicates that the TX FIFO needs writing
        TXW OFFSET(17) NUMBITS(1) [],
        /// Indicates that the RX FIFO needs reading
        RXR OFFSET(18) NUMBITS(1) [],
        /// Indicates that the TX FIFO can accept data
        TXD OFFSET(19) NUMBITS(1) [],
        /// Indicates that the RX FIFO contains data
        RXD OFFSET(20) NUMBITS(1) [],
        /// TX FIFO is empty
        TXE OFFSET(21) NUMBITS(1) [],
        /// RX FIFO is full
        RXF OFFSET(22) NUMBITS(1) [],
        /// RX sign extend
        RXSEX OFFSET(23) NUMBITS(1) [],
        /// PCM clock sync helper
        SYNC OFFSET(24) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub FIFO_A [
        /// Data written here is transmitted, and received data is read from
        /// here
        FIFO OFFSET(0) NUMBITS(32) [],
    ]
}

register_bitfields! {u32,
    pub MODE_A [
        /// Frame sync length
        FSLEN OFFSET(0) NUMBITS(10) [],
        /// Frame length
        FLEN OFFSET(10) NUMBITS(10) [],
        /// Frame sync invert
        FSI OFFSET(20) NUMBITS(1) [],
        /// Frame sync mode
        FSM OFFSET(21) NUMBITS(1) [
            Master = 0,
            Slave = 1,
        ],
        /// Clock invert
        CLKI OFFSET(22) NUMBITS(1) [],
        /// PCM clock mode
        CLKM OFFSET(23) NUMBITS(1) [
            Master = 0,
            Slave = 1,
        ],
        /// Transmit frame packed mode
        FTXP OFFSET(24) NUMBITS(1) [
            Unpacked = 0,
            Packed16x2 = 1,
        ],
        /// Receive frame packed mode
        FRXP OFFSET(25) NUMBITS(1) [
            Unpacked = 0,
            Packed16x2 = 1,
        ],
        /// PDM input mode enable
        PDME OFFSET(26) NUMBITS(1) [
            Pcm = 0,
            Pdm = 1,
        ],
        /// PDM decimation factor
        PDMN OFFSET(27) NUMBITS(1) [
            Sixteen = 0,
            ThirtyTwo = 1,
        ],
        CLK_DIS OFFSET(28) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub RXC_A [
        /// Channel 2 width
        CH2WID OFFSET(0) NUMBITS(4) [],
        /// Channel 2 position
        CH2POS OFFSET(4) NUMBITS(10) [],
        /// Channel 2 enable
        CH2EN OFFSET(14) NUMBITS(1) [],
        /// Channel 2 width extension bit
        CH2WEX OFFSET(15) NUMBITS(1) [],
        /// Channel 1 width
        CH1WID OFFSET(16) NUMBITS(4) [],
        /// Channel 1 position
        CH1POS OFFSET(20) NUMBITS(10) [],
        /// Channel 1 enable
        CH1EN OFFSET(30) NUMBITS(1) [],
        /// Channel 1 width extension bit
        CH1WEX OFFSET(31) NUMBITS(1) [],
    ]
}

pub use RXC_A as TXC_A;

register_bitfields! {u32,
    pub DREQ_A [
        /// RX request level
        RX_REQ OFFSET(0) NUMBITS(7) [],
        /// TX request level
        TX_REQ OFFSET(8) NUMBITS(7) [],
        /// RX panic level
        RX_PANIC OFFSET(16) NUMBITS(7) [],
        /// TX panic level
        TX_PANIC OFFSET(24) NUMBITS(7) [],
    ]
}

register_bitfields! {u32,
    pub INTEN_A [
        /// TX write interrupt enable
        TXW OFFSET(0) NUMBITS(1) [],
        /// RX read interrupt enable
        RXR OFFSET(1) NUMBITS(1) [],
        /// TX error interrupt
        TXERR OFFSET(2) NUMBITS(1) [],
        /// RX error interupt
        RXERR OFFSET(3) NUMBITS(1) [],
    ]
}

pub use INTEN_A as INTSTC_A;

register_bitfields! {u32,
    pub GRAY [
        /// Clear the GRAY mode logic
        CLR OFFSET(1) NUMBITS(1) [],
        /// Flush the RX buffer into the RX FIFO
        FLUSH OFFSET(2) NUMBITS(1) [],
        /// The current fill level of the RX buffer
        RXLEVEL OFFSET(4) NUMBITS(6) [],
        /// The number of bits that were flushed into the RX fifo
        FLUSHED OFFSET(10) NUMBITS(6) [],
        /// The current level of the RX FIFO
        RXFIFOLEVEL OFFSET(16) NUMBITS(6) [],
    ]
}

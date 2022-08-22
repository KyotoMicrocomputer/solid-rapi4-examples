//! [BCM2711 SPI][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A136%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

use crate::Vpa;

/// The base address of the SPI0 instance of [the SPI register block](Registers).
pub const BASE_SPI0: Vpa = Vpa(0x4_7e20_4000);
/// The base address of the SPI3 instance of [the SPI register block](Registers).
pub const BASE_SPI3: Vpa = Vpa(0x4_7e20_4600);
/// The base address of the SPI4 instance of [the SPI register block](Registers).
pub const BASE_SPI4: Vpa = Vpa(0x4_7e20_4800);
/// The base address of the SPI5 instance of [the SPI register block](Registers).
pub const BASE_SPI5: Vpa = Vpa(0x4_7e20_4a00);
/// The base address of the SPI6 instance of [the SPI register block](Registers).
pub const BASE_SPI6: Vpa = Vpa(0x4_7e20_4c00);

register_structs! {
    pub Registers {
        /// SPI master control and status
        (0x00 => pub cs: ReadWrite<u32, CS::Register>),
        /// SPI master TX and RX FIFOs
        (0x04 => pub fifo: ReadWrite<u32, FIFO::Register>),
        /// SPI master clock divider
        (0x08 => pub clk: ReadWrite<u32, CLK::Register>),
        /// SPI master data length
        (0x0c => pub dlen: ReadWrite<u32, DLEN::Register>),
        /// SPI LoSSI mode TOH
        (0x10 => pub ltoh: ReadWrite<u32, LTOH::Register>),
        /// SPI DMA DREQ controls
        (0x14 => pub dc: ReadWrite<u32, DC::Register>),
        (0x18 => @END),
    }
}

register_bitfields! {u32,
    pub CS [
        /// Chip select
        CS OFFSET(0) NUMBITS(2) [
            ChipSelect0 = 0b00,
            ChipSelect1 = 0b01,
            ChipSelect2 = 0b10,
        ],
        /// Clock phase
        CPHA OFFSET(2) NUMBITS(1) [
            FirstSclkTransitionAtMiddleOfDataBit = 0,
            FirstSclkTransitionAtBeginningOFDataBit = 1,
        ],
        /// Clock polarity
        CPOL OFFSET(3) NUMBITS(1) [
            RestStateIsLow = 0,
            RestStateIsHigh = 1,
        ],
        /// TX FIFO clear
        CLEAR_TX OFFSET(4) NUMBITS(1) [],
        /// RX FIFO clear
        CLEAR_RX OFFSET(5) NUMBITS(1) [],
        /// Chip select polarity
        CSPOL OFFSET(6) NUMBITS(1) [
            ActiveLow = 0,
            ActiveHigh = 1,
        ],
        /// Transfer active
        TA OFFSET(7) NUMBITS(1) [],
        /// DMA enable
        DMAEN OFFSET(8) NUMBITS(1) [],
        /// Interrupt on done
        INTD OFFSET(9) NUMBITS(1) [],
        /// Interrupt on RXR
        INTR OFFSET(10) NUMBITS(1) [],
        /// Automatically de-assert chip select
        ADCS OFFSET(11) NUMBITS(1) [],
        /// Read enable
        REN OFFSET(12) NUMBITS(1) [],
        /// LoSSI enable
        LEN OFFSET(13) NUMBITS(1) [
            Spi = 0,
            Lossi = 1,
        ],
        /// Unused
        LMONO OFFSET(14) NUMBITS(1) [],
        /// Unused
        TE_EN OFFSET(15) NUMBITS(1) [],
        /// Transfer done
        DONE OFFSET(16) NUMBITS(1) [],
        /// RX FIFO contains data
        RXD OFFSET(17) NUMBITS(1) [],
        /// TX FIFO can accept data
        TXD OFFSET(18) NUMBITS(1) [],
        /// RX FIFO needs reading
        RXR OFFSET(19) NUMBITS(1) [],
        /// RX FIFO full
        RXF OFFSET(20) NUMBITS(1) [],
        /// Chip select 0 polarity
        CSPOL0 OFFSET(21) NUMBITS(1) [
            ActiveLow = 0,
            ActiveHigh = 1,
        ],
        /// Chip select 1 polarity
        CSPOL1 OFFSET(22) NUMBITS(1) [
            ActiveLow = 0,
            ActiveHigh = 1,
        ],
        /// Chip select 2 polarity
        CSPOL2 OFFSET(23) NUMBITS(1) [
            ActiveLow = 0,
            ActiveHigh = 1,
        ],
        /// Enable DMA mode in LoSSI mode
        DMA_LEN OFFSET(24) NUMBITS(1) [],
        /// Enable long data word in LoSSI mode if `DMA_LEN` is set
        LEN_LONG OFFSET(25) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub FIFO [
        /// Read from RX FIFO or write to TX FIFO
        ///
        /// *DMA Mode (`DMAEN` set):* If `TA` is clear, the first 32-bit write
        /// to this register will control `SPIDLEN` and `SPICS`. Subsequent
        /// reads and writes will be taken as four-byte data words to be read
        /// or written to the FIFOs.
        ///
        /// *Poll/Interrupt Mode (`DMAEN` clear, `TA` set):* Writes to the
        /// register write bytes to the TX FIFO. Reads from the register read
        /// bytes from the RX FIFO.
        DATA OFFSET(0) NUMBITS(32) [],
    ]
}

register_bitfields! {u32,
    pub CLK [
        /// Clock divider
        CDIV OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub DLEN [
        /// Data length: The number of bytes to transfer.
        LEN OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub LTOH [
        /// This sets the Output Hold delay in APB clocks. A value of `0` causes
        /// a one-clock delay.
        TOH OFFSET(0) NUMBITS(4) [],
    ]
}

register_bitfields! {u32,
    pub DC [
        /// DMA write request threshold
        TDREQ OFFSET(0) NUMBITS(8) [],
        /// DMA write panic threshold
        TPANIC OFFSET(8) NUMBITS(8) [],
        /// DMA read request threshold
        RDREQ OFFSET(16) NUMBITS(8) [],
        /// DMA read panic threshold
        RPANIC OFFSET(24) NUMBITS(8) [],
    ]
}

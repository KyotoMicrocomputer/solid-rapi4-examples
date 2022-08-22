//! [PL011 UART][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A147%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::Vpa;

/// The base address of the UART0 instance of [the PL011 register block](Registers).
pub const BASE_UART0: Vpa = Vpa(0x4_7e20_1000);
/// The base address of the UART2 instance of [the PL011 register block](Registers).
pub const BASE_UART2: Vpa = Vpa(0x4_7e20_1400);
/// The base address of the UART3 instance of [the PL011 register block](Registers).
pub const BASE_UART3: Vpa = Vpa(0x4_7e20_1600);
/// The base address of the UART4 instance of [the PL011 register block](Registers).
pub const BASE_UART4: Vpa = Vpa(0x4_7e20_1800);
/// The base address of the UART5 instance of [the PL011 register block](Registers).
pub const BASE_UART5: Vpa = Vpa(0x4_7e20_1a00);

register_structs! {
    pub Registers {
        /// Data Register
        (0x00 => pub dr: ReadWrite<u32, DR::Register>),
        /// Receive status and error clear register
        (0x04 => pub rsrecr: ReadWrite<u32, RSRECR::Register>),
        (0x08 => _pad0),
        /// Flag register
        (0x18 => pub fr: ReadOnly<u32, FR::Register>),
        (0x1c => _pad1),
        /// Integer baud rate divisor
        (0x24 => pub ibrd: ReadWrite<u32, IBRD::Register>),
        /// Fractional baud rate divisor
        (0x28 => pub fbrd: ReadWrite<u32, FBRD::Register>),
        /// Line control register
        (0x2c => pub lcrh: ReadWrite<u32, LCRH::Register>),
        /// Control register
        (0x30 => pub cr: ReadWrite<u32, CR::Register>),
        /// Interrupt FIFO level select register
        (0x34 => pub ifls: ReadWrite<u32, IFLS::Register>),
        /// Interrupt mask set clear register
        (0x38 => pub imsc: ReadWrite<u32, IMSC::Register>),
        /// Raw interrupt status register
        (0x3c => pub ris: ReadOnly<u32, RIS::Register>),
        /// Masked interrupt status register
        (0x40 => pub mis: ReadOnly<u32, MIS::Register>),
        /// Interrupt clear register
        (0x44 => pub icr: WriteOnly<u32, ICR::Register>),
        /// DMA control register
        (0x48 => pub dmacr: ReadWrite<u32, DMACR::Register>),
        (0x4c => _pad2),
        /// Test control register
        (0x80 => pub itcr: ReadWrite<u32, ITCR::Register>),
        /// Integration test input register
        (0x84 => pub itip: ReadWrite<u32, ITIP::Register>),
        /// Integration test output register
        (0x88 => pub itop: ReadWrite<u32, ITOP::Register>),
        /// Test data register
        (0x8c => pub tdr: ReadWrite<u32, TDR::Register>),
        (0x90 => @END),
    }
}

register_bitfields! {u32,
    pub DR [
        /// Receive/transmit data
        DATA OFFSET(0) NUMBITS(8) [],
        /// Framing error (read-only).
        FE OFFSET(8) NUMBITS(1) [],
        /// Parity error (read-only).
        PE OFFSET(9) NUMBITS(1) [],
        /// Break error (read-only).
        BE OFFSET(10) NUMBITS(1) [],
        /// Overrun error (read-only).
        OE OFFSET(11) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub RSRECR [
        /// Framing error
        FE OFFSET(0) NUMBITS(1) [],
        /// Parity error
        PE OFFSET(1) NUMBITS(1) [],
        /// Break error
        BE OFFSET(2) NUMBITS(1) [],
        /// Overrun error
        OE OFFSET(3) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub FR [
        /// Clear to send
        CTS OFFSET(0) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DSR OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DCD OFFSET(2) NUMBITS(1) [],
        /// UART busy
        BUSY OFFSET(3) NUMBITS(1) [],
        /// Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [],
        /// Transmit FIFO full
        TXFF OFFSET(5) NUMBITS(1) [],
        /// Receive FIFO full
        RXFF OFFSET(6) NUMBITS(1) [],
        /// Transmit FIFO empty
        TXFE OFFSET(7) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        RI OFFSET(8) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub IBRD [
        /// The integer baud rate divisor
        IBRD OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub FBRD [
        /// The fractional baud rate divisor
        FBRD OFFSET(0) NUMBITS(6) [],
    ]
}

register_bitfields! {u32,
    pub LCRH [
        /// Send break
        BRK OFFSET(0) NUMBITS(1) [],
        /// Parity enable
        PEN OFFSET(1) NUMBITS(1) [],
        /// Even parity select
        EPS OFFSET(2) NUMBITS(1) [
            Odd = 0,
            Even = 1,
        ],
        /// Two stop bits select
        STP2 OFFSET(3) NUMBITS(1) [],
        /// Enable FIFOs
        FEN OFFSET(4) NUMBITS(1) [],
        /// Word length
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBits = 0b00,
            SixBits = 0b01,
            SevenBits = 0b10,
            EightBits = 0b11,
        ],
        /// Stick parity select
        SPS OFFSET(7) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub CR [
        /// UART enable
        UARTEN OFFSET(0) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        SIREN OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        SIRLP OFFSET(2) NUMBITS(1) [],
        /// Loopback enable
        LBE OFFSET(7) NUMBITS(1) [],
        /// Transmit enable
        TXE OFFSET(8) NUMBITS(1) [],
        /// Receive enable
        RXE OFFSET(9) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DTR OFFSET(10) NUMBITS(1) [],
        /// Request to send
        RTS OFFSET(11) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        OUT1 OFFSET(12) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        OUT2 OFFSET(13) NUMBITS(1) [],
        /// RTS hardware flow control enable
        RTSEN OFFSET(14) NUMBITS(1) [],
        /// CTS hardware flow control enable
        CTSEN OFFSET(15) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub IFLS [
        /// Transmit interrupt FIFO level select
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            /// Trigger an interrupt when the FIFO becomes 1/8 full
            OneEighth = 0b000,
            /// Trigger an interrupt when the FIFO becomes 1/4 full
            OneQuarter = 0b001,
            /// Trigger an interrupt when the FIFO becomes 1/2 full
            OneHalf = 0b010,
            /// Trigger an interrupt when the FIFO becomes 3/4 full
            ThreeQuarters = 0b011,
            /// Trigger an interrupt when the FIFO becomes 7/8 full
            SevenEighths = 0b100,
        ],
        /// Receive interrupt FIFO level select
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            /// Trigger an interrupt when the FIFO becomes 1/8 full
            OneEighth = 0b000,
            /// Trigger an interrupt when the FIFO becomes 1/4 full
            OneQuarter = 0b001,
            /// Trigger an interrupt when the FIFO becomes 1/2 full
            OneHalf = 0b010,
            /// Trigger an interrupt when the FIFO becomes 3/4 full
            ThreeQuarters = 0b011,
            /// Trigger an interrupt when the FIFO becomes 7/8 full
            SevenEighths = 0b100,
        ],
        /// Unsupported, write zero, read as don't care
        TXIFPSEL OFFSET(6) NUMBITS(3) [],
        /// Unsupported, write zero, read as don't care
        RXIFPSEL OFFSET(9) NUMBITS(3) [],
    ]
}

register_bitfields! {u32,
    pub IMSC [
        /// Unsupported, write zero, read as don't care
        RIMIM OFFSET(0) NUMBITS(1) [],
        /// `nUARTCTS` modem interrupt mask
        CTSMIM OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DCDMIM OFFSET(2) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DSRMIM OFFSET(3) NUMBITS(1) [],
        /// Receive interrupt mask
        RXIM OFFSET(4) NUMBITS(1) [],
        /// Transmit interrupt mask
        TXIM OFFSET(5) NUMBITS(1) [],
        /// Receive timeout interrupt mask
        RTIM OFFSET(6) NUMBITS(1) [],
        /// Framing error interrupt mask
        FEIM OFFSET(7) NUMBITS(1) [],
        /// Parity error interrupt mask
        PEIM OFFSET(8) NUMBITS(1) [],
        /// Break error interrupt mask
        BEIM OFFSET(9) NUMBITS(1) [],
        /// Overrun error interrupt mask
        OEIM OFFSET(10) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub RIS [
        /// Unsupported, write zero, read as don't care
        RIRMIS OFFSET(0) NUMBITS(1) [],
        /// `nUARTCTS` modem interrupt status
        CTSRMIS OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DCDRMIS OFFSET(2) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DSRRMIS OFFSET(3) NUMBITS(1) [],
        /// Receive interrupt status
        RXRIS OFFSET(4) NUMBITS(1) [],
        /// Transmit interrupt status
        TXRIS OFFSET(5) NUMBITS(1) [],
        /// Receive timeout interrupt status
        RTRIS OFFSET(6) NUMBITS(1) [],
        /// Framing error interrupt status
        FERIS OFFSET(7) NUMBITS(1) [],
        /// Parity error interrupt status
        PERIS OFFSET(8) NUMBITS(1) [],
        /// Break error interrupt status
        BERIS OFFSET(9) NUMBITS(1) [],
        /// Overrun error interrupt status
        OERIS OFFSET(10) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub MIS [
        /// Unsupported, write zero, read as don't care
        RIMMIS OFFSET(0) NUMBITS(1) [],
        /// `nUARTCTS` modem masked interrupt status
        CTSMMIS OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DCDMMIS OFFSET(2) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DSRMMIS OFFSET(3) NUMBITS(1) [],
        /// Receive masked interrupt status
        RXMIS OFFSET(4) NUMBITS(1) [],
        /// Transmit masked interrupt status
        TXMIS OFFSET(5) NUMBITS(1) [],
        /// Receive timeout masked interrupt status
        RTMIS OFFSET(6) NUMBITS(1) [],
        /// Framing error masked interrupt status
        FEMIS OFFSET(7) NUMBITS(1) [],
        /// Parity error masked interrupt status
        PEMIS OFFSET(8) NUMBITS(1) [],
        /// Break error masked interrupt status
        BEMIS OFFSET(9) NUMBITS(1) [],
        /// Overrun error masked interrupt status
        OEMIS OFFSET(10) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub ICR [
        /// Unsupported, write zero, read as don't care
        RIMIC OFFSET(0) NUMBITS(1) [],
        /// `nUARTCTS` modem interrupt clear
        CTSMIC OFFSET(1) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DCDMIC OFFSET(2) NUMBITS(1) [],
        /// Unsupported, write zero, read as don't care
        DSRMIC OFFSET(3) NUMBITS(1) [],
        /// Receive interrupt clear
        RXIC OFFSET(4) NUMBITS(1) [],
        /// Transmit interrupt clear
        TXIC OFFSET(5) NUMBITS(1) [],
        /// Receive timeout interrupt clear
        RTIC OFFSET(6) NUMBITS(1) [],
        /// Framing error interrupt clear
        FEIC OFFSET(7) NUMBITS(1) [],
        /// Parity error interrupt clear
        PEIC OFFSET(8) NUMBITS(1) [],
        /// Break error interrupt clear
        BEIC OFFSET(9) NUMBITS(1) [],
        /// Overrun error interrupt clear
        OEIC OFFSET(10) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub DMACR [
        /// Receive DMA enable
        RXDMAE OFFSET(0) NUMBITS(1) [],
        /// Transmit DMA enable
        TXDMAE OFFSET(1) NUMBITS(1) [],
        /// DMA on error
        DMAONERR OFFSET(2) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub ITCR [
        /// Integration test enable
        ITCR0 OFFSET(0) NUMBITS(1) [],
        /// Test FIFO enable
        ITCR1 OFFSET(1) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub ITIP [
        /// Reads return the value of the `UARTRXD` primary input.
        ITIP0 OFFSET(0) NUMBITS(1) [],
        /// Reads return the value of the `nUARTCTS` primary input.
        ITIP3 OFFSET(3) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub ITOP [
        ITOP0 OFFSET(0) NUMBITS(1) [],
        ITOP3 OFFSET(3) NUMBITS(1) [],
        ITOP6 OFFSET(6) NUMBITS(1) [],
        ITOP7 OFFSET(7) NUMBITS(1) [],
        ITOP8 OFFSET(8) NUMBITS(1) [],
        ITOP9 OFFSET(9) NUMBITS(1) [],
        ITOP10 OFFSET(10) NUMBITS(1) [],
        ITOP11 OFFSET(11) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub TDR [
        TDR10_0 OFFSET(0) NUMBITS(11) [],
    ]
}

//! [Auxiliaries][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A11%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::Vpa;

/// The base address of [the AUX Mini UART register block](Registers).
pub const BASE: Vpa = Vpa(0x4_7e21_5000);

register_structs! {
    pub Registers {
        (0x00 => pub aux_irq: ReadOnly<u32, AUX_IRQ::Register>),
        (0x04 => pub aux_enables: ReadWrite<u32, AUX_ENABLES::Register>),
        (0x08 => _pad0),
        (0x40 => pub aux_mu: MiniUartRegisters),
        (0x80 => pub aux_spi1: SpiRegisters),
        (0xc0 => pub aux_spi2: SpiRegisters),
        (0x100 => @END),
    },

    pub MiniUartRegisters {
        (0x00 => pub io_reg: ReadWrite<u32, AUX_MU_IO_REG::Register>),
        (0x04 => pub ier_reg: ReadWrite<u32, AUX_MU_IER_REG::Register>),
        (0x08 => pub iir_reg: ReadWrite<u32, AUX_MU_IIR_REG::Register>),
        (0x0c => pub lcr_reg: ReadWrite<u32, AUX_MU_LCR_REG::Register>),
        (0x10 => pub mcr_reg: ReadWrite<u32, AUX_MU_MCR_REG::Register>),
        (0x14 => pub lsr_reg: ReadOnly<u32, AUX_MU_LSR_REG::Register>),
        (0x18 => pub msr_reg: ReadOnly<u32, AUX_MU_MSR_REG::Register>),
        (0x1c => pub scratch: ReadWrite<u32, AUX_MU_SCRATCH::Register>),
        (0x20 => pub cntl_reg: ReadWrite<u32, AUX_MU_CNTL_REG::Register>),
        (0x24 => pub stat_reg: ReadOnly<u32, AUX_MU_STAT_REG::Register>),
        (0x28 => pub baud_reg: ReadWrite<u32, AUX_MU_BAUD_REG::Register>),
        (0x2c => _pad0),
        (0x40 => @END),
    },

    pub SpiRegisters {
        (0x00 => pub cntl0_reg: ReadWrite<u32, AUX_SPI_CNTL0_REG::Register>),
        (0x04 => pub cntl1_reg: ReadWrite<u32, AUX_SPI_CNTL1_REG::Register>),
        (0x08 => pub stat_reg: ReadOnly<u32, AUX_SPI_STAT_REG::Register>),
        (0x0c => pub peek_reg: ReadOnly<u32, AUX_SPI_DATA::Register>),
        (0x10 => _pad0),
        (0x20 => pub io_rega: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x24 => pub io_regb: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x28 => pub io_regc: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x2c => pub io_regd: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x30 => pub txhold_rega: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x34 => pub txhold_regb: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x38 => pub txhold_regc: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x3c => pub txhold_regd: ReadWrite<u32, AUX_SPI_DATA::Register>),
        (0x40 => @END),
    }
}

register_bitfields! {u32,
    pub AUX_IRQ [
        /// If set the mini UART has an interrupt pending.
        MINI_UART_IRQ OFFSET(0) NUMBITS(1) [],
        /// If set the SPI1 module has an interrupt pending.
        SPI1_IRQ OFFSET(1) NUMBITS(1) [],
        /// If set the SPI2 module has an interrupt pending.
        SPI2_IRQ OFFSET(2) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub AUX_ENABLES [
        /// If set the mini UART is enabled.
        MINI_UART_ENABLE OFFSET(0) NUMBITS(1) [],
        /// If set the SPI1 module is enabled.
        SPI1_ENABLE OFFSET(1) NUMBITS(1) [],
        /// If set the SPI2 module is enabled.
        SPI2_ENABLE OFFSET(2) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_IO_REG [
        /// Receive data read, `DLAB` = 0
        RX_DATA OFFSET(0) NUMBITS(8) [],
        /// Transmit data write, `DLAB` = 0
        TX_DATA OFFSET(8) NUMBITS(8) [],
        /// LS 8 bits baudrate, `DLAB` = 1
        BAUDRATE_LO OFFSET(16) NUMBITS(8) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_IER_REG [
        /// Enable transmit interrupt, `DLAB` = 0
        TX_INT_ENABLE OFFSET(0) NUMBITS(1) [],
        /// Enable receive interrupt, `DLAB` = 0
        RX_INT_ENABLE OFFSET(1) NUMBITS(1) [],
        /// MS 8 bits baudrate, `DLAB` = 1
        BAUDRATE_HI OFFSET(0) NUMBITS(8) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_IIR_REG [
        /// Interrupt pending
        INT_PENDING OFFSET(0) NUMBITS(1) [],
        /// TX interrupt ID bit (read), FIFO clear (write)
        TX_INT_PENDING_FIFO_CLEAR OFFSET(1) NUMBITS(1) [],
        /// RX interrupt ID bit (write), FIFO clear (read)
        RX_INT_PENDING_FIFO_CLEAR OFFSET(2) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_LCR_REG [
        /// Data size
        DATA_SIZE OFFSET(0) NUMBITS(1) [
            SevenBits = 7,
            EightBits = 8,
        ],
        /// Break
        BREAK OFFSET(6) NUMBITS(1) [],
        /// DLAB access - if set the first two Mini UART registers give access
        /// to the Baudrate register.
        DLAB OFFSET(7) NUMBITS(1) [
            Normal = 0,
            RouteToBaudrateReg = 1,
        ],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_MCR_REG [
        /// RTS signal (if set `UART1_RTS` pin is low)
        RTS OFFSET(1) NUMBITS(1) [
            High = 0,
            Low = 1,
        ],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_LSR_REG [
        /// Data ready
        DATA_READY OFFSET(0) NUMBITS(1) [],
        /// Receiver overrun (RO)
        RX_OVERRUN OFFSET(1) NUMBITS(1) [],
        /// Transmitter empty
        TX_EMPTY OFFSET(5) NUMBITS(1) [],
        /// Transmitter idle
        TX_IDLE OFFSET(6) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_MSR_REG [
        /// CTS status (if set `UART1_CTS` pin is low)
        CTS OFFSET(4) NUMBITS(1) [
            High = 0,
            Low = 1,
        ],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_SCRATCH [
        /// Scratch
        SCRATCH OFFSET(0) NUMBITS(8) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_CNTL_REG [
        /// Receiver enable
        RX_ENABLE OFFSET(0) NUMBITS(1) [],
        /// Transmitter enable
        TX_ENABLE OFFSET(1) NUMBITS(1) [],
        /// Enable receive auto flow-control using RTS
        RTS_AFC OFFSET(2) NUMBITS(1) [],
        /// Enable transmit auto flow-control using CTS
        CTS_AFC OFFSET(3) NUMBITS(1) [],
        // RTS auto flow level
        RTS_AFC_LEVEL OFFSET(4) NUMBITS(2) [
            Three = 0b00,
            Two = 0b01,
            One = 0b10,
            Four = 0b11,
        ],
        /// RTS assert level
        RTS_AFC_POLARITY OFFSET(6) NUMBITS(1) [],
        /// CTS assert level
        CTS_AFC_POLARITY OFFSET(7) NUMBITS(1) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_STAT_REG [
        /// Symbol available
        RX_NOT_EMPTY OFFSET(0)  NUMBITS(1) [],
        /// Space available
        TX_NOT_FULL OFFSET(1) NUMBITS(1) [],
        /// Receiver is idle
        RX_IDLE OFFSET(2) NUMBITS(1) [],
        /// Transmitter is idle
        TX_IDLE OFFSET(3) NUMBITS(1) [],
        /// Receiver overrun
        RX_OVERRUN OFFSET(4) NUMBITS(1) [],
        /// Transmit FIFO is full
        TX_FULL OFFSET(5) NUMBITS(1) [],
        /// RTS status
        RTS OFFSET(6) NUMBITS(1) [],
        /// CTS line
        CTS OFFSET(7) NUMBITS(1) [],
        /// Transmit FIFO is empty
        TX_EMPTY OFFSET(8) NUMBITS(1) [],
        /// Transmitter done
        TX_DONE OFFSET(9) NUMBITS(1) [],
        /// Receive FIFO fill level (`0..=8`)
        RX_FIFO_LEVEL OFFSET(16) NUMBITS(4) [],
        /// Transmit FIFO fill level (`0..=8`)
        TX_FIFO_LEVEL OFFSET(24) NUMBITS(4) [],
    ]
}

register_bitfields! {u32,
    pub AUX_MU_BAUD_REG [
        /// Baudrate
        BAUDRATE OFFSET(0) NUMBITS(16) [],
    ]
}

register_bitfields! {u32,
    pub AUX_SPI_CNTL0_REG [
        /// Shift length
        SHIFT_LEN OFFSET(0) NUMBITS(6) [],
        /// SHift out MS bit first
        SHIFT_OUT_DIR OFFSET(6) NUMBITS(1) [
            LsbFirst = 0,
            MsbFirst = 1,
        ],
        /// Invert SPI CLK
        CLK_POLARITY OFFSET(7) NUMBITS(1) [
            IdleLow = 0,
            IdleHigh = 1,
        ],
        /// Out rising
        OUT_EDGE OFFSET(8) NUMBITS(1) [
            FallingEdge = 0,
            RisingEdge = 1,
        ],
        /// Clear FIFOs
        CLEAR_FIFO OFFSET(9) NUMBITS(1) [],
        /// In rising
        IN_EDGE OFFSET(10) NUMBITS(1) [
            FallingEdge = 0,
            RisingEdge = 1,
        ],
        /// Enable
        ENABLE OFFSET(11) NUMBITS(1) [],
        /// DOUT hold time
        DOUT_HOLD_TIME OFFSET(12) NUMBITS(2) [
            Zero = 0b00,
            One = 0b01,
            Four = 0b10,
            Seven = 0b11,
        ],
        /// Variable width
        VARIABLE_WIDTH OFFSET(14) NUMBITS(1) [],
        /// Variable CS
        VARIABLE_CS OFFSET(15) NUMBITS(1) [],
        /// Post-input mode
        POST_INPUT OFFSET(16) NUMBITS(1) [],
        /// Chip selects
        CS OFFSET(17) NUMBITS(3) [],
        /// Speed: `spi_clk_freq = system_clock_freq / 2 * (speed + 1)`
        SPEED OFFSET(20) NUMBITS(12) [],
    ]
}

register_bitfields! {u32,
    pub AUX_SPI_CNTL1_REG [
        /// Keep input
        KEEP_INPUT OFFSET(0) NUMBITS(1) [],
        /// SHift in MS bit first
        SHIFT_IN_DIR OFFSET(1) NUMBITS(1) [
            LsbFirst = 0,
            MsbFirst = 1,
        ],
        /// Done IRQ
        DONE_IRQ OFFSET(6) NUMBITS(1) [],
        /// TX empty IRQ
        TX_EMPTY_IRQ OFFSET(7) NUMBITS(1) [],
        /// CS high time
        CS_HIGH_TIME OFFSET(8) NUMBITS(3) [],
    ]
}

register_bitfields! {u32,
    pub AUX_SPI_STAT_REG [
        /// Bit count
        BIT_COUNT OFFSET(0) NUMBITS(6) [],
        /// Busy
        BUSY OFFSET(6) NUMBITS(1) [],
        /// RX empty
        RX_EMPTY OFFSET(7) NUMBITS(1) [],
        /// RX full
        RX_FULL OFFSET(8) NUMBITS(1) [],
        /// TX empty
        TX_EMPTY OFFSET(9) NUMBITS(1) [],
        /// TX full
        TX_FULL OFFSET(10) NUMBITS(1) [],
        /// RX fifo level
        RX_FIFO_LEVEL OFFSET(16) NUMBITS(4) [],
        /// TX fifo level
        TX_FIFO_LEVEL OFFSET(24) NUMBITS(4) [],
    ]
}

register_bitfields! {u32,
    pub AUX_SPI_DATA [
        DATA OFFSET(0) NUMBITS(16) [],
    ]
}

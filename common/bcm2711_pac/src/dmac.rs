//! [DMA controller][1]
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A34%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_bitfields, register_structs,register_bitmasks,
    registers::{ReadOnly, ReadWrite},
    fields::Field,
    RegisterLongName,
};

use crate::{MemoryField, Vpa};

/// The base address of [the DMA0-14 register block](Dma0Registers).
pub const BASE_DMA0: Vpa = Vpa(0x4_7e00_7000);

/// The number of DMA engines in this SoC.
pub const COUNT: usize = 16;

register_structs! {
    /// DMA0-14 register block
    pub Dma0Registers {
        /// DMA0-6
        (0x000 => pub dma0: [DmaRegisters; 7]),
        /// DMA7-10
        (0x700 => pub dma7: [DmaLiteRegisters; 4]),
        /// DMA11-14
        (0xb00 => pub dma11: [Dma4Registers; 4]),
        (0xf00 => _pad0),
        /// Interrupt status of each DMA channel
        (0xfe0 => pub int_status: ReadOnly<u32, INT_STATUS::Register>),
        (0xfe4 => _pad1),
        /// Global enable bits for each DMA channel
        (0xff0 => pub enable: ReadWrite<u32, ENABLE::Register>),
        (0xff4 => @END),
    },

    /// A DMA engine (DMA0-6) register block
    pub DmaRegisters {
        /// Control and status
        (0x00 => pub cs: ReadWrite<u32, DMA_CS::Register>),
        /// Control block address
        (0x04 => pub conblk_ad: ReadWrite<u32>),
        /// CB word 0 (transfer information)
        (0x08 => pub ti: ReadWrite<u32, DMA_TI::Register>),
        /// CB word 1 (source address)
        (0x0c => pub source_ad: ReadWrite<u32>),
        /// CB word 2 (destination address)
        (0x10 => pub dest_ad: ReadWrite<u32>),
        /// CB word 3 (transfer length)
        (0x14 => pub txfr_len: ReadWrite<u32, DMA_TXFR_LEN::Register>),
        /// CB word 4 (2D stride)
        (0x18 => pub stride: ReadWrite<u32, DMA_STRIDE::Register>),
        /// CB word 5 (next CB address)
        (0x1c => pub nextconbk: ReadWrite<u32>),
        /// Debug
        (0x20 => pub debug: ReadWrite<u32, DMA_DEBUG::Register>),
        (0x24 => _pad1),
        (0x100 => @END),
    },

    /// A DMA Lite (DMA7-10) engine register block
    pub DmaLiteRegisters {
        /// Control and status
        (0x00 => pub cs: ReadWrite<u32, DMA_LITE_CS::Register>),
        /// Control block address
        (0x04 => pub conblk_ad: ReadWrite<u32>),
        /// CB word 0 (transfer information)
        (0x08 => pub ti: ReadWrite<u32, DMA_LITE_TI::Register>),
        /// CB word 1 (source address)
        (0x0c => pub source_ad: ReadWrite<u32>),
        /// CB word 2 (destination address)
        (0x10 => pub dest_ad: ReadWrite<u32>),
        /// CB word 3 (transfer length)
        (0x14 => pub txfr_len: ReadWrite<u32, DMA_LITE_TXFR_LEN::Register>),
        (0x18 => _pad0),
        /// CB word 5 (next CB address)
        (0x1c => pub nextconbk: ReadWrite<u32>),
        /// Debug
        (0x20 => pub debug: ReadWrite<u32, DMA_LITE_DEBUG::Register>),
        (0x24 => _pad1),
        (0x100 => @END),
    },

    /// A *DMA4* (DMA11-14) register block
    pub Dma4Registers {
        /// Control and status
        (0x00 => pub cs: ReadWrite<u32, DMA4_CS::Register>),
        /// Control block address
        (0x04 => pub cb: ReadWrite<u32>),
        (0x08 => _pad0),
        /// Debug
        (0x0c => pub debug: ReadWrite<u32, DMA4_DEBUG::Register>),
        /// CB word 0 (transfer information)
        (0x10 => pub ti: ReadWrite<u32, DMA4_TI::Register>),
        /// CB word 1 (source address)
        (0x14 => pub src: ReadWrite<u32>),
        /// CB word 2 (source information)
        (0x18 => pub srci: ReadWrite<u32, DMA4_SRCI::Register>),
        /// CB word 3 (destination address)
        (0x1c => pub dest: ReadWrite<u32>),
        /// CB word 4 (destination information)
        (0x20 => pub desti: ReadWrite<u32, DMA4_DESTI::Register>),
        /// CB word 5 (transfer length)
        (0x24 => pub len: ReadWrite<u32, DMA4_LEN::Register>),
        /// CB word 6 (next CB address)
        (0x28 => pub next_cb: ReadWrite<u32>),
        /// More debug
        (0x2c => pub debug2: ReadOnly<u32, DMA4_DEBUG2::Register>),
        (0x30 => _pad1),
        (0x100 => @END),
    }
}

impl Dma0Registers {
    /// DMA0
    #[inline]
    pub const fn dma0(&self) -> &DmaRegisters {
        &self.dma0[0]
    }
    
    /// DMA1
    #[inline]
    pub const fn dma1(&self) -> &DmaRegisters {
        &self.dma0[1]
    }
    
    /// DMA2
    #[inline]
    pub const fn dma2(&self) -> &DmaRegisters {
        &self.dma0[2]
    }
    
    /// DMA3
    #[inline]
    pub const fn dma3(&self) -> &DmaRegisters {
        &self.dma0[3]
    }
    
    /// DMA4
    #[inline]
    pub const fn dma4(&self) -> &DmaRegisters {
        &self.dma0[4]
    }
    
    /// DMA5
    #[inline]
    pub const fn dma5(&self) -> &DmaRegisters {
        &self.dma0[5]
    }
    
    /// DMA6
    #[inline]
    pub const fn dma6(&self) -> &DmaRegisters {
        &self.dma0[6]
    }
    
    /// DMA7
    #[inline]
    pub const fn dma7(&self) -> &DmaLiteRegisters {
        &self.dma7[0]
    }
    
    /// DMA8
    #[inline]
    pub const fn dma8(&self) -> &DmaLiteRegisters {
        &self.dma7[1]
    }
    
    /// DMA9
    #[inline]
    pub const fn dma9(&self) -> &DmaLiteRegisters {
        &self.dma7[2]
    }
    
    /// DMA10
    #[inline]
    pub const fn dma10(&self) -> &DmaLiteRegisters {
        &self.dma7[3]
    }
    
    /// DMA11
    #[inline]
    pub const fn dma11(&self) -> &Dma4Registers {
        &self.dma11[0]
    }

    /// DMA12
    #[inline]
    pub const fn dma12(&self) -> &Dma4Registers {
        &self.dma11[1]
    }

    /// DMA13
    #[inline]
    pub const fn dma13(&self) -> &Dma4Registers {
        &self.dma11[2]
    }

    /// DMA14
    #[inline]
    pub const fn dma14(&self) -> &Dma4Registers {
        &self.dma11[3]
    }
}

/// DMA engine control block
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct DmaCb {
    /// Transfer information
    pub ti: MemoryField<u32, DMA_TI::Register>,
    /// Source address
    pub source_ad: u32,
    /// Destination address
    pub dest_ad: u32,
    /// Transfer length
    pub txfr_len: MemoryField<u32, DMA_TXFR_LEN::Register>,
    /// 2D stride
    pub stride: MemoryField<u32, DMA_STRIDE::Register>,
    /// Next CB address
    pub nextconbk: u32,
    /// Reserved - set to zero
    pub reserved: [u32; 2],
}

/// DMA Lite engine control block
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct DmaLiteCb {
    /// Transfer information
    pub ti: MemoryField<u32, DMA_LITE_TI::Register>,
    /// Source address
    pub source_ad: u32,
    /// Destination address
    pub dest_ad: u32,
    /// Transfer length
    pub txfr_len: MemoryField<u32, DMA_LITE_TXFR_LEN::Register>,
    /// Reserved - set to zero
    pub reserved0: u32,
    /// Next CB address
    pub nextconbk: u32,
    /// Reserved - set to zero
    pub reserved1: [u32; 2],
}

/// DMA4 engine control block
#[derive(Default, Debug, Copy, Clone)]
#[repr(C)]
pub struct Dma4Cb {
    /// Transfer information
    pub ti: MemoryField<u32, DMA4_TI::Register>,
    /// Source address
    pub src: u32,
    /// Source information
    pub srci: MemoryField<u32, DMA4_SRCI::Register>,
    /// Destination address
    pub dest: u32,
    /// Destination information
    pub desti: MemoryField<u32, DMA4_DESTI::Register>,
    /// Transfer length
    pub len: MemoryField<u32, DMA4_LEN::Register>,
    /// Next CB address
    pub next_cb: u32,
    /// Reserved - set to zero
    pub reserved: u32,
}

#[allow(non_snake_case)]
pub mod INT_STATUS {
    use super::*;
    pub struct Register;
    impl RegisterLongName for Register {}

    /// Construct a [`Field`] representing the `INT` bit (interrupt status, RO)
    /// corresponding to the specified DMA engine instance.
    ///
    /// # Panic
    ///
    /// Panics if `i` is outside the range `0..`[`COUNT`].
    #[inline]
    pub const fn INT(i: usize) -> Field<u32, Register> {
        assert!(i < COUNT);
        Field::new(0b1, i)
    }
}

#[allow(non_snake_case)]
pub mod ENABLE {
    use super::*;
    pub struct Register;
    impl RegisterLongName for Register {}

    /// Construct a [`Field`] representing the `EN` bit (enable, RW)
    /// corresponding to the specified DMA engine instance.
    ///
    /// # Panic
    ///
    /// Panics if `i` is outside the range `0..14`.
    #[inline]
    pub const fn EN(i: usize) -> Field<u32, Register> {
        assert!(i < 14);
        Field::new(0b1, i)
    }
    
    register_bitmasks!(u32, Register, [
        /// Set the 1G SDRAM ram page that the 32-bit DMA engines (DMA0-6) will
        /// access when addressing the 1G uncached range
        /// `0xc000_0000..=0xffff_ffff`
        PAGE OFFSET(24) NUMBITS(4) [],
        /// Set the 1G SDRAM ram page that the DMA Lite engines (DMA7-10) will
        /// access when addressing the 1G uncached range
        /// `0xc000_0000..=0xffff_ffff`
        PAGELITE OFFSET(28) NUMBITS(4) [],
    ]);
}

register_bitfields! {u32,
    pub DMA_CS [
        /// Acitvate the DMA
        ACTIVE OFFSET(0) NUMBITS(1) [],
        /// DMA end flag (W1C)
        END OFFSET(1) NUMBITS(1) [],
        /// Interrupt status (W1C)
        INT OFFSET(2) NUMBITS(1) [],
        /// `DREQ` state (RO)
        DREQ OFFSET(3) NUMBITS(1) [],
        /// DMA paused state (RO)
        PAUSED OFFSET(4) NUMBITS(1) [],
        /// DMA paused by `DREQ` state (RO)
        DREQ_STOPS_DMA OFFSET(5) NUMBITS(1) [],
        /// DMA is waiting for the last write to be received (RO)
        WAITING_FOR_OUTSTANDING_WRITES OFFSET(6) NUMBITS(1) [],
        /// DMA error (RO)
        ERROR OFFSET(8) NUMBITS(1) [],
        /// AXI priority level
        PRIORITY OFFSET(16) NUMBITS(4) [],
        /// AXI panic priority level
        PANIC_PRIORITY OFFSET(20) NUMBITS(4) [],
        /// Wait for outstanding writes
        WAIT_FOR_OUTSTANDING_WRITES OFFSET(28) NUMBITS(1) [],
        /// Disable debug pause signal
        DISDEBUG OFFSET(29) NUMBITS(1) [],
        /// Abort DMA (W1SC)
        ABORT OFFSET(30) NUMBITS(1) [],
        /// DMA channel reset (W1SC)
        RESET OFFSET(31) NUMBITS(1) [],
    ],
    pub DMA_TI [
        /// Interrupt enable
        INTEN OFFSET(0) NUMBITS(1) [],
        /// 2D mode
        TDMODE OFFSET(1) NUMBITS(1) [
            Linear = 0,
            TwoD = 1,
        ],
        /// Wait for a write response
        WAIT_RESP OFFSET(3) NUMBITS(1) [],
        /// Destination address increment
        DEST_INC OFFSET(4) NUMBITS(1) [],
        /// Destination transfer width
        DEST_WIDTH OFFSET(5) NUMBITS(1) [
            ThirtyTwoBits = 0,
            OneHundredAndTwentyEightBits = 1,
        ],
        /// Control destination writes with `DREQ`
        DEST_DREQ OFFSET(6) NUMBITS(1) [],
        /// Ignore writes
        DEST_IGNORE OFFSET(7) NUMBITS(1) [],
        /// Source address increment
        SRC_INC OFFSET(8) NUMBITS(1) [],
        /// Source transfer width
        SRC_WIDTH OFFSET(9) NUMBITS(1) [
            ThirtyTwoBits = 0,
            OneHundredAndTwentyEightBits = 1,
        ],
        /// Control source reads with DREQ
        SRC_DREQ OFFSET(10) NUMBITS(1) [],
        /// Ignore reads
        SRC_IGNORE OFFSET(11) NUMBITS(1) [],
        /// Burst transfer length
        BURST_LENGTH OFFSET(12) NUMBITS(4) [],
        /// Peripheral mapping
        PERMAP OFFSET(16) NUMBITS(5) [],
        /// Add wait cycles
        WAITS OFFSET(21) NUMBITS(5) [],
        /// Don't do wide writes as a two-beat burst
        NO_WIDE_BURSTS OFFSET(26) NUMBITS(1) [],
    ],
    pub DMA_TXFR_LEN [
        /// (Linear mode) Transfer length in bytes
        LENGTH OFFSET(0) NUMBITS(30) [],
        /// (2D mode) X transfer length in bytes
        XLENGTH OFFSET(0) NUMBITS(16) [],
        /// (2D mode) Y transfer length, indicating how many `XLENGTH` transfers
        /// are performed
        YLENGTH OFFSET(16) NUMBITS(14) [],
    ],
    pub DMA_STRIDE [
        /// Source stride in bytes (signed)
        S_STRIDE OFFSET(0) NUMBITS(16) [],
        /// Destination stride in bytes (signed)
        D_STRIDE OFFSET(16) NUMBITS(16) [],
    ],
    pub DMA_DEBUG [
        /// Read last not set error (W1C)
        READ_LAST_NOT_SET_ERROR OFFSET(0) NUMBITS(1) [],
        /// FIFO error (W1C)
        FIFO_ERROR OFFSET(1) NUMBITS(1) [],
        /// Slave read response error (W1C)
        READ_ERROR OFFSET(2) NUMBITS(1) [],
        /// DMA outstanding writes counter (RO)
        OUTSTANDING_WRITES OFFSET(4) NUMBITS(4) [],
        /// DMA AXI ID (RO)
        DMA_ID OFFSET(8) NUMBITS(8) [],
        /// DMA state machine state (RO)
        DMA_STATE OFFSET(16) NUMBITS(9) [],
        /// DMA version (RO)
        VERSION OFFSET(25) NUMBITS(3) [],
        /// DMA Lite (RO)
        LITE OFFSET(28) NUMBITS(1) [],
    ],

    pub DMA_LITE_TI [
        /// Interrupt enable
        INTEN OFFSET(0) NUMBITS(1) [],
        /// Wait for a write response
        WAIT_RESP OFFSET(3) NUMBITS(1) [],
        /// Destination address increment
        DEST_INC OFFSET(4) NUMBITS(1) [],
        /// Destination transfer width
        DEST_WIDTH OFFSET(5) NUMBITS(1) [
            ThirtyTwoBits = 0,
            OneHundredAndTwentyEightBits = 1,
        ],
        /// Control destination writes with `DREQ`
        DEST_DREQ OFFSET(6) NUMBITS(1) [],
        /// Source address increment
        SRC_INC OFFSET(8) NUMBITS(1) [],
        /// Source transfer width
        SRC_WIDTH OFFSET(9) NUMBITS(1) [
            ThirtyTwoBits = 0,
            OneHundredAndTwentyEightBits = 1,
        ],
        /// Control source reads with DREQ
        SRC_DREQ OFFSET(10) NUMBITS(1) [],
        /// Burst transfer length
        BURST_LENGTH OFFSET(12) NUMBITS(4) [],
        /// Peripheral mapping
        PERMAP OFFSET(16) NUMBITS(5) [],
        /// Add wait cycles
        WAITS OFFSET(21) NUMBITS(5) [],
    ],
    pub DMA_LITE_TXFR_LEN [
        /// Transfer length in bytes
        LENGTH OFFSET(0) NUMBITS(30) [],
    ],

    pub DMA4_CS [
        /// Activate the DMA4
        ACTIVE OFFSET(0) NUMBITS(1) [],
        /// End flag (W1C)
        END OFFSET(1) NUMBITS(1) [],
        /// Interrupt status (W1C)
        INT OFFSET(2) NUMBITS(1) [],
        /// DREQ state (RO)
        DREQ OFFSET(3) NUMBITS(1) [],
        /// DMA read paused state (RO)
        RD_PAUSED OFFSET(4) NUMBITS(1) [],
        /// DMA write paused state (RO)
        WR_PAUSED OFFSET(5) NUMBITS(1) [],
        /// DMA paused by DREQ state (RO)
        DREQ_STOPS_DMA OFFSET(6) NUMBITS(1) [],
        /// The DMA4 is waiting for all the write response to be returned (RO)
        WAITING_FOR_OUTSTANDING_WRITES OFFSET(7) NUMBITS(1) [],
        /// DMA error (RO)
        ERROR OFFSET(10) NUMBITS(1) [],
        /// AXI QoS Level
        QOS OFFSET(16) NUMBITS(4) [],
        /// AXI panic QoS level
        PANIC_QOS OFFSET(20) NUMBITS(4) [],
        /// Indicates the DMA4 is busy (RO)
        DMA_BUSY OFFSET(24) NUMBITS(1) [],
        /// Indicates that there are outstanding AXI transfers, either
        /// outstanding read data or outstanding write responses (RO)
        OUTSTANDING_TRANSACTIONS OFFSET(25) NUMBITS(1) [],
        /// Wait for outstanding writes
        WAIT_FOR_OUTSTANDING_WRITES OFFSET(28) NUMBITS(1) [],
        /// Disable debug pause signal
        DISDEBUG OFFSET(29) NUMBITS(1) [],
        /// Abort DMA (W1SC)
        ABORT OFFSET(30) NUMBITS(1) [],
    ],
    pub DMA4_DEBUG [
        /// Slave write response error (RC)
        WRITE_ERROR OFFSET(0) NUMBITS(1) [],
        /// FIFO error (RC)
        FIFO_ERROR OFFSET(1) NUMBITS(1) [],
        /// Slave read response error (RC)
        READ_ERROR OFFSET(2) NUMBITS(1) [],
        /// Slave read response error during control block read (RC)
        READ_CB_ERROR OFFSET(3) NUMBITS(1) [],
        /// Generate an interrupt if an error is detected
        INT_ON_ERROR OFFSET(8) NUMBITS(1) [],
        /// Instruct the DMA4 to halt if it detects an error
        HALT_ON_ERROR OFFSET(9) NUMBITS(1) [],
        /// Instruct the DMA4 to abort if it detects an error
        ABORT_ON_ERROR OFFSET(10) NUMBITS(1) [],
        /// Disalbe the clock gating logic
        DISABLE_CLK_GATE OFFSET(11) NUMBITS(1) [],
        /// Read state machine state (RO)
        R_STATE OFFSET(14) NUMBITS(4) [
            Idle = 0,
            WaitCbData = 1,
            Calc = 2,
            Read4k = 3,
            Reading = 4,
            ReadFifoFull = 5,
            WaitWriteComplete = 6,
        ],
        /// Write state machine state (RO)
        W_STATE OFFSET(18) NUMBITS(4) [
            Idle = 0,
            Preload = 1,
            Calc = 2,
            Write4k = 3,
            ReadFifoEmpty = 4,
            WaitOutstanding = 5,
        ],
        /// DMA reset (W1SC)
        RESET OFFSET(23) NUMBITS(1) [],
        /// ID
        ID OFFSET(24) NUMBITS(4) [],
        /// DMA version (RO)
        VERSION OFFSET(28) NUMBITS(4) [],
    ],
    pub DMA4_TI [
        /// Interrupt enable
        INTEN OFFSET(0) NUMBITS(1) [],
        /// 2D mode
        TDMODE OFFSET(1) NUMBITS(1) [
            Linear = 0,
            TwoD = 1,
        ],
        /// Wait for a write response
        WAIT_RESP OFFSET(2) NUMBITS(1) [],
        /// Wait for a read response
        WAIT_RD_RESP OFFSET(3) NUMBITS(1) [],
        /// Peripheral mapping
        PERMAP OFFSET(9) NUMBITS(5) [],
        /// Control source reads with `DREQ`
        S_DREQ OFFSET(14) NUMBITS(1) [],
        /// Control destination reads with `DREQ`
        D_DREQ OFFSET(15) NUMBITS(1) [],
        /// Read wait cycles
        S_WAITS OFFSET(16) NUMBITS(8) [],
        /// Write wait cycles
        D_WAITS OFFSET(24) NUMBITS(8) [],
    ],
    pub DMA4_SRCI [
        /// High bits of the address `[40:32]`
        ADDR OFFSET(0) NUMBITS(8) [],
        /// Burst transfer length
        BURST_LENGTH OFFSET(8) NUMBITS(4) [],
        /// Increment the source address
        INC OFFSET(12) NUMBITS(1) [],
        /// Transfer width
        SIZE OFFSET(13) NUMBITS(2) [
            ThirtyTwo = 0b00,
            SixtyFour = 0b01,
            OneHundredAndTwentyEight = 0b10,
            TwoHundredsAndFiftySix = 0b11,
        ],
        /// Ignoe reads or writes
        IGNORE OFFSET(15) NUMBITS(1) [],
        /// 2D mode stride in bytes (signed)
        STRIDE OFFSET(16) NUMBITS(16) [],
    ],
    pub DMA4_LEN [
        /// (Linear mode) Transfer length in bytes
        LENGTH OFFSET(0) NUMBITS(30) [],
        /// (2D mode) X transfer length in bytes
        XLENGTH OFFSET(0) NUMBITS(16) [],
        /// (2D mode) Y transfer length minus one, indicating how many
        /// `XLENGTH` transfers are performed
        YLENGTH OFFSET(16) NUMBITS(14) [],
    ],
    pub DMA4_DEBUG2 [
        /// Outstanding write response count
        OUTSTANDING_WRITES OFFSET(0) NUMBITS(9) [],
        /// Outstanding read words count
        OUTSTANDING_READS OFFSET(0) NUMBITS(9) [],
    ],
}

pub use DMA4_SRCI as DMA4_DESTI;
pub use DMA_CS as DMA_LITE_CS;
pub use DMA_DEBUG as DMA_LITE_DEBUG;

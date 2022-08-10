//! [BCM2711 ARM timer][1] (**based** on ARM SP804)
//!
//! [1]: https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf#%5B%7B%22num%22%3A162%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22XYZ%22%7D%2C115%2C841.89%2Cnull%5D
use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::Vpa;

/// The base address of [the ARM timer register block](Registers).
pub const BASE: Vpa = Vpa(0x4_7e00_b400);

register_structs! {
    pub Registers {
        /// Load
        (0x00 => pub load: ReadWrite<u32>),
        /// Value (Read-Only)
        (0x04 => pub value: ReadOnly<u32>),
        /// Control
        (0x08 => pub control: ReadWrite<u32, CONTROL::Register>),
        /// IRQ Clear/Ack (Write-Only)
        (0x0c => pub irqcntl: WriteOnly<u32, IRQCNTL::Register>),
        /// RAW IRQ (Read-Only)
        (0x10 => pub rawirq: ReadOnly<u32, RAWIRQ::Register>),
        /// Masked IRQ (Read-Only)
        (0x14 => pub mskirq: ReadOnly<u32, MSKIRQ::Register>),
        /// Reload
        (0x18 => pub reload: ReadWrite<u32>),
        /// Pre-divider (Not in real 804!)
        (0x1c => pub prediv: ReadWrite<u32, PREDIV::Register>),
        /// Free running counter (Not in real 804!)
        (0x20 => pub freecnt: ReadOnly<u32>),
        (0x24 => @END),
    }
}

register_bitfields! {u32,
    pub CONTROL [
        _32BIT OFFSET(1) NUMBITS(1) [
            SixteenBitCounters = 0,
            ThirtyTwoBitCounter = 1,
        ],
        /// Pre-scale bits
        DIV OFFSET(2) NUMBITS(2) [
            DivideBy1 = 0b00,
            DivideBy16 = 0b01,
            DivideBy256 = 0b10,
        ],
        /// Timer interrupt enable
        IE OFFSET(5) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        /// Timer enable
        ENABLE OFFSET(7) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        /// Halt in debug halted mode
        DBGHALT OFFSET(8) NUMBITS(1) [
            KeepRunning = 0,
            Halt = 1,
        ],
        /// Free running counter enable
        ENAFREE OFFSET(9) NUMBITS(1) [
            Disable = 0,
            Enable = 1,
        ],
        /// Free running counter pre-scaler. Freq is `sys_clk / (prescale + 1)`
        FREEDIV OFFSET(16) NUMBITS(8) [],
    ]
}

register_bitfields! {u32,
    pub IRQCNTL [
        INT OFFSET(0) NUMBITS(1) [
            Clear = 1,
        ]
    ]
}

register_bitfields! {u32,
    pub RAWIRQ [
        INT OFFSET(0) NUMBITS(1) []
    ]
}

register_bitfields! {u32,
    pub MSKIRQ [
        INT OFFSET(0) NUMBITS(1) []
    ]
}

register_bitfields! {u32,
    pub PREDIV [
        /// Pre-divider value. `timer_clock = apb_clock / (pre_divider + 1)`
        PREDIV OFFSET(0) NUMBITS(10) []
    ]
}

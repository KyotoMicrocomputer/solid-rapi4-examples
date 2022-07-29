use tock_registers::{
    fields::{Field, FieldValue},
    register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
    RegisterLongName,
};

use crate::Vpa;

/// The base address of [the GPIO register block](Registers).
pub const BASE: Vpa = Vpa(0x4_7e20_0000);

register_structs! {
    pub Registers {
        /// GPIO function select
        (0x00 => pub gpfsel: [ReadWrite<u32, GPFSEL::Register>; 6]),
        (0x18 => _reserved9),
        /// GPIO pin output set
        (0x1c => pub gpset: [WriteOnly<u32, GPSET::Register>; 2]),
        (0x24 => _reserved10),
        /// GPIO pin output clear
        (0x28 => pub gpclr: [WriteOnly<u32, GPCLR::Register>; 2]),
        (0x30 => _reserved0),
        /// GPIO pin level
        (0x34 => pub gplev: [ReadOnly<u32, GPLEV::Register>; 2]),
        (0x3c => _reserved1),
        /// GPIO pin event detect status
        (0x40 => pub gpeds: [ReadWrite<u32, GPEDS::Register>; 2]),
        (0x48 => _reserved2),
        /// GPIO pin rising edge detect enable
        (0x4c => pub gpren: [ReadWrite<u32, GPREN::Register>; 2]),
        (0x54 => _reserved3),
        /// GPIO pin falling edge detect enable
        (0x58 => pub gpfen: [ReadWrite<u32, GPFEN::Register>; 2]),
        (0x60 => _reserved4),
        /// GPIO pin high detect enable
        (0x64 => pub gphen: [ReadWrite<u32, GPHEN::Register>; 2]),
        (0x6c => _reserved5),
        /// GPIO pin low detect enable
        (0x70 => pub gplen: [ReadWrite<u32, GPLEN::Register>; 2]),
        (0x78 => _reserved6),
        /// GPIO pin asynchronous rising edge detect enable
        (0x7c => pub gparen: [ReadWrite<u32, GPAREN::Register>; 2]),
        (0x84 => _reserved7),
        /// GPIO pin asynchronous falling edge detect enable
        (0x88 => pub gpafen: [ReadWrite<u32, GPAFEN::Register>; 2]),
        (0x90 => _reserved8),
        /// GPIO pull-up/pull-down register
        (0xe4 => pub gpio_pup_pdn_cntrl_reg: [ReadWrite<u32>; 4]),
        (0xf4 => @END),
    }
}

/// GPIO function select
#[allow(non_snake_case)]
pub mod GPFSEL {
    use super::*;
    pub struct Register;
    impl RegisterLongName for Register {}

    /// The number of pins represented by each `GPFSEL` register.
    pub const PINS_PER_REGISTER: usize = 10;

    /// Field value: Use the pin as an input
    pub const INPUT: u32 = 0b000;
    /// Field value: Use the pin as an output
    pub const OUTPUT: u32 = 0b001;
    /// Field value: The pin takes alternate function 0
    pub const ALT0: u32 = 0b100;
    /// Field value: The pin takes alternate function 1
    pub const ALT1: u32 = 0b101;
    /// Field value: The pin takes alternate function 2
    pub const ALT2: u32 = 0b110;
    /// Field value: The pin takes alternate function 3
    pub const ALT3: u32 = 0b111;
    /// Field value: The pin takes alternate function 4
    pub const ALT4: u32 = 0b011;
    /// Field value: The pin takes alternate function 5
    pub const ALT5: u32 = 0b010;

    /// Construct a [`Field`] corresponding to the specified pin number.
    ///
    /// # Panic
    ///
    /// Panics if `i` is outside the range `0..`[`PINS_PER_REGISTER`].
    #[inline]
    pub const fn pin(i: usize) -> Field<u32, Register> {
        assert!(i < PINS_PER_REGISTER);
        Field::new(0b111, 3 * i)
    }
}

#[macropol::macropol]
macro_rules! register_pin_field {
    (
        $( #[$meta:meta] )*
        pub mod $NAME:ident {
            $(
                #[field_value($value:expr)]
                $( #[$value_meta:meta] )*
                pub const fn $value_name:ident();
            )*
        }
    ) => {
        $( #[$meta] )*
        #[allow(non_snake_case)]
        pub mod $NAME {
            use super::*;
            pub struct Register;
            impl RegisterLongName for Register {}

            /// The number of pins represented by each `$&NAME` register.
            pub const PINS_PER_REGISTER: usize = 32;

            /// Construct a [`Field`] corresponding to the specified pin number.
            ///
            /// # Panic
            ///
            /// Panics if `i` is outside the range `0..`[`PINS_PER_REGISTER`].
            #[inline]
            pub const fn pin(i: usize) -> Field<u32, Register> {
                assert!(i < PINS_PER_REGISTER);
                Field::new(0b1, i)
            }

            $(
                $( #[$value_meta] )*
                ///
                /// # Panic
                ///
                /// Panics if `i` is outside the range `0..`[`PINS_PER_REGISTER`].
                #[inline]
                pub const fn $value_name(i: usize) -> FieldValue<u32, Register> {
                    assert!(i < PINS_PER_REGISTER);
                    FieldValue::<u32, _>::new(0b1, i, $value)
                }
            )*
        }
    };
}

register_pin_field! {
    pub mod pin_generic {}
}

pub use pin_generic as GPLEV;
pub use pin_generic as GPEDS;
pub use pin_generic as GPREN;
pub use pin_generic as GPFEN;
pub use pin_generic as GPHEN;
pub use pin_generic as GPLEN;
pub use pin_generic as GPAREN;
pub use pin_generic as GPAFEN;

register_pin_field! {
    /// GPIO pin output set
    pub mod GPSET {
        #[field_value(1)]
        /// Construct a [`FieldValue`] that can be used to set the output of the
        /// specified pin.
        pub const fn set();
    }
}

register_pin_field! {
    /// GPIO pin output clear
    pub mod GPCLR {
        #[field_value(1)]
        /// Construct a [`FieldValue`] that can be used to clear the output of the
        /// specified pin.
        pub const fn clear();
    }
}

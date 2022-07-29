#![feature(const_option)] // `Option::unwrap` in a constant context
use bcm2711_pac::gpio;
use core::cell::UnsafeCell;
use tock_registers::interfaces::{ReadWriteable, Writeable};

#[inline]
fn gpio_regs() -> &'static gpio::Registers {
    // Safety: SOLID for RaPi4B provides an identity mapping in this area, and we don't alter
    // the mapping
    unsafe { &*(gpio::BASE.to_arm_pa().unwrap() as usize as *const gpio::Registers) }
}

const GPIO_NUM: usize = 42;

#[no_mangle]
pub extern "C" fn rust_entry() {
    // Configure the GPIO pin for output
    gpio_regs().gpfsel[GPIO_NUM / gpio::GPFSEL::PINS_PER_REGISTER].modify(
        gpio::GPFSEL::pin(GPIO_NUM % gpio::GPFSEL::PINS_PER_REGISTER).val(gpio::GPFSEL::OUTPUT),
    );

    // Register a timer
    assert_eq!(
        unsafe { solid_abi::SOLID_TIMER_RegisterTimer(&TIMER_HANDLER) },
        0,
    );
}

static TIMER_HANDLER: solid_abi::SOLID_TIMER_HANDLER = solid_abi::SOLID_TIMER_HANDLER {
    pNext: UnsafeCell::new(std::ptr::null_mut()),
    pCallQ: UnsafeCell::new(std::ptr::null_mut()),
    globalTick: UnsafeCell::new(0),
    r#type: solid_abi::SOLID_TIMER_TYPE_INTERVAL,
    time: 200_000, // usec
    func: timer_handler,
    param: std::ptr::null_mut(),
};

unsafe extern "C" fn timer_handler(_: *mut (), _: *mut ()) {
    static mut STATE: bool = false;
    STATE = !STATE;
    if STATE {
        gpio_regs().gpset[GPIO_NUM / gpio::GPSET::PINS_PER_REGISTER]
            .write(gpio::GPSET::set(GPIO_NUM % gpio::GPSET::PINS_PER_REGISTER));
    } else {
        gpio_regs().gpclr[GPIO_NUM / gpio::GPCLR::PINS_PER_REGISTER].write(gpio::GPCLR::clear(
            GPIO_NUM % gpio::GPCLR::PINS_PER_REGISTER,
        ));
    }
}

/// SOLID Core Services low-level binding
#[allow(non_snake_case)]
mod solid_abi {
    use core::cell::UnsafeCell;

    extern "C" {
        pub fn SOLID_TIMER_RegisterTimer(pHandler: *const SOLID_TIMER_HANDLER) -> i32;
    }

    pub const SOLID_TIMER_TYPE_INTERVAL: i32 = 1;

    #[repr(C)]
    pub struct SOLID_TIMER_HANDLER {
        pub pNext: UnsafeCell<*mut Self>,
        pub pCallQ: UnsafeCell<*mut Self>,
        pub globalTick: UnsafeCell<u64>,
        pub r#type: i32,
        pub time: u32,
        pub func: unsafe extern "C" fn(param: *mut (), ctx: *mut ()),
        pub param: *mut (),
    }

    unsafe impl Sync for SOLID_TIMER_HANDLER {}
}

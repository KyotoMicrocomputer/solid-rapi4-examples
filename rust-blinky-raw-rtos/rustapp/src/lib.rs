#[no_mangle]
pub extern "C" fn slo_main() {
    unsafe { ffi::SOLID_LOG_printf(b"Starting LED blinker\n\0".as_ptr().cast()) };

    // Configure the LED port
    green_led::init();

    loop {
        // Turn on the LED
        green_led::update(true);
        unsafe { ffi::dly_tsk(200_000) };

        // Turn off the LED
        green_led::update(false);
        unsafe { ffi::dly_tsk(200_000) };
    }
}

/// FFI declarations used in this application
#[allow(non_camel_case_types)]
mod ffi {
    use std::os::raw::{c_char, c_int};

    pub type int_t = c_int;

    pub type RELTIM = u32;
    pub type ER = int_t;

    extern "C" {
        pub fn SOLID_LOG_printf(format: *const c_char, ...);
        pub fn dly_tsk(dlytim: RELTIM) -> ER;
    }
}

mod green_led {
    const GPIO_BASE: usize = 0xFE200000;
    const GPIO_NUM: usize = 42;

    pub fn init() {
        unsafe {
            let reg = (GPIO_BASE + 0x00 /* GPFSEL0 */ + (GPIO_NUM / 10) * 4) as *mut u32;
            let mode = 1; // output
            reg.write_volatile(
                reg.read_volatile() & !(7 << (GPIO_NUM % 10 * 3)) | (mode << (GPIO_NUM % 10 * 3)),
            );
        }
    }

    pub fn update(new_state: bool) {
        unsafe {
            let reg = (GPIO_BASE
                + if new_state {
                    0x1c /* GPSET0 */
                } else {
                    0x28 /* GPCLR0 */
                }
                + (GPIO_NUM / 32) * 4) as *mut u32;
            reg.write_volatile(1 << (GPIO_NUM % 32));
        }
    }
}

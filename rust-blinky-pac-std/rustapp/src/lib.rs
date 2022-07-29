#![feature(const_option)] // `Option::unwrap` in a constant context
use bcm2711_pac::gpio;
use std::{thread::sleep, time::Duration};
use tock_registers::interfaces::{ReadWriteable, Writeable};

#[inline]
fn gpio_regs() -> &'static gpio::Registers {
    // Safety: SOLID for RaPi4B provides an identity mapping in this area, and we don't alter
    // the mapping
    unsafe { &*(gpio::BASE.to_arm_pa().unwrap() as usize as *const gpio::Registers) }
}

#[no_mangle]
pub extern "C" fn rust_entry() {
    let gpio_regs = gpio_regs();
    let gpio_num = 42;

    // Configure the GPIO pin for output
    gpio_regs.gpfsel[gpio_num / gpio::GPFSEL::PINS_PER_REGISTER].modify(
        gpio::GPFSEL::pin(gpio_num % gpio::GPFSEL::PINS_PER_REGISTER).val(gpio::GPFSEL::OUTPUT),
    );

    loop {
        // Turn on the LED
        gpio_regs.gpset[gpio_num / gpio::GPSET::PINS_PER_REGISTER]
            .write(gpio::GPSET::set(gpio_num % gpio::GPSET::PINS_PER_REGISTER));
        sleep(Duration::from_millis(200));

        // Turn off the LED
        gpio_regs.gpclr[gpio_num / gpio::GPCLR::PINS_PER_REGISTER].write(gpio::GPCLR::clear(
            gpio_num % gpio::GPCLR::PINS_PER_REGISTER,
        ));
        sleep(Duration::from_millis(200));
    }
}

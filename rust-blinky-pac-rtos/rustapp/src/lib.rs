use itron::{task::delay, time::duration};

#[no_mangle]
pub extern "C" fn rust_entry() {
    println!("Starting LED blinker");

    // Configure the LED port
    green_led::init();

    loop {
        // Turn on the LED
        green_led::update(true);
        delay(duration!(ms: 200)).unwrap();

        // Turn off the LED
        green_led::update(false);
        delay(duration!(ms: 200)).unwrap();
    }
}

mod green_led {
    use bcm2711_pac::gpio;
    use tock_registers::interfaces::{ReadWriteable, Writeable};

    const GPIO_NUM: usize = 42;

    fn gpio_regs() -> &'static gpio::Registers {
        // Safety: SOLID for RaPi4B provides an identity mapping in this area, and we don't alter
        // the mapping
        unsafe { &*(gpio::BASE.to_arm_pa().unwrap() as usize as *const gpio::Registers) }
    }

    pub fn init() {
        // Configure the GPIO pin for output
        gpio_regs().gpfsel[GPIO_NUM / gpio::GPFSEL::PINS_PER_REGISTER].modify(
            gpio::GPFSEL::pin(GPIO_NUM % gpio::GPFSEL::PINS_PER_REGISTER).val(gpio::GPFSEL::OUTPUT),
        );
    }

    pub fn update(new_state: bool) {
        if new_state {
            gpio_regs().gpset[GPIO_NUM / gpio::GPSET::PINS_PER_REGISTER]
                .write(gpio::GPSET::set(GPIO_NUM % gpio::GPSET::PINS_PER_REGISTER));
        } else {
            gpio_regs().gpclr[GPIO_NUM / gpio::GPCLR::PINS_PER_REGISTER].write(gpio::GPCLR::clear(
                GPIO_NUM % gpio::GPCLR::PINS_PER_REGISTER,
            ));
        }
    }
}

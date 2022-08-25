#[no_mangle]
pub extern "C" fn slo_main() {
    println!("Starting LED blinker");

    // Configure the LED port
    green_led::init();

    // Tracks the latest LED state. Moved into the handler closure when
    // we create `solid::timer::Timer`.
    let mut state = false;

    // Construct a timer object on heap
    //
    // (There are ways to do this on a global variable, but we would need either
    // unsafe code or incomplete unstable features to do this ergonomically for now.)
    let mut timer = Box::pin(solid::timer::Timer::new(
        solid::timer::Schedule::Interval(solid::timer::Usecs32(200_000)),
        move |_: solid::thread::CpuCx<'_>| {
            // Determine the next LED state
            state = !state;

            // Toggle the LED
            green_led::update(state);
        },
    ));

    // Start the timer
    assert!(
        timer.as_mut().start().expect("unable to start timer"),
        "timer was already running"
    );

    assert!(timer.is_running());

    // Keep the timer alive
    std::mem::forget(timer);
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

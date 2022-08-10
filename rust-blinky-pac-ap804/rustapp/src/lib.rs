#[no_mangle]
pub extern "C" fn rust_entry() {
    println!("Starting LED blinker");

    // Configure the LED port
    green_led::init();

    // Configure the AP804 instance
    ap804::init(1_000_000);

    // Tracks the latest LED state. Moved into the handler closure when
    // we create `solid::interrupt::Handler`.
    let mut state = false;

    // Construct an interrupt handler object on heap
    //
    // (There are ways to do this on a global variable, but we would need either
    // unsafe code or incomplete unstable features to do this ergonomically for now.)
    let handler = Box::new(solid::interrupt::Handler::new(
        move |_: solid::thread::CpuCx<'_>| {
            // Clear the AP804 instance's interrupt flag
            ap804::clear_int();

            // Determine the next LED state
            state = !state;

            // Toggle the LED
            green_led::update(state);
        },
    ));

    // Keep the interrupt handler alive indefinitely
    let handler = std::pin::Pin::static_mut(Box::leak(handler));

    // Register the interrupt handler
    assert!(
        handler
            .register_static(&solid::interrupt::HandlerOptions::new(ap804::INTNO, 10))
            .expect("unable to register interrupt handler"),
        "interrupt handler was already registered"
    );

    // Enable the AP804 interrupt line
    ap804::INTNO
        .enable()
        .expect("unable to enable interrupt line");

    // Start the AP804 timer
    ap804::start();
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

mod ap804 {
    use bcm2711_pac::ap804;
    use tock_registers::interfaces::Writeable;

    pub const INTNO: solid::interrupt::Number = solid::interrupt::Number(64);

    fn ap804_regs() -> &'static ap804::Registers {
        // Safety: SOLID for RaPi4B provides an identity mapping in this area, and we don't alter
        // the mapping
        unsafe { &*(ap804::BASE.to_arm_pa().unwrap() as usize as *const ap804::Registers) }
    }

    pub fn init(load: u32) {
        ap804_regs().control.write(
            ap804::CONTROL::_32BIT::ThirtyTwoBitCounter
                + ap804::CONTROL::DIV::DivideBy1
                + ap804::CONTROL::IE::Disable
                + ap804::CONTROL::ENABLE::Disable
                + ap804::CONTROL::DBGHALT::KeepRunning
                + ap804::CONTROL::ENAFREE::Disable
                + ap804::CONTROL::FREEDIV.val(0x3e),
        );
        ap804_regs().load.set(load);
        ap804_regs().reload.set(load);
        ap804_regs().prediv.write(ap804::PREDIV::PREDIV.val(0x7d));
        ap804_regs().irqcntl.write(ap804::IRQCNTL::INT::Clear);
    }

    pub fn start() {
        ap804_regs().control.write(
            ap804::CONTROL::_32BIT::ThirtyTwoBitCounter
                + ap804::CONTROL::DIV::DivideBy1
                + ap804::CONTROL::IE::Enable // <--
                + ap804::CONTROL::ENABLE::Enable // <--
                + ap804::CONTROL::DBGHALT::KeepRunning
                + ap804::CONTROL::ENAFREE::Disable
                + ap804::CONTROL::FREEDIV.val(0x3e),
        );
    }

    pub fn clear_int() {
        ap804_regs().irqcntl.write(ap804::IRQCNTL::INT::Clear);
    }
}

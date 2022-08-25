use futures::FutureExt;
use std::time::Duration;
use tokio::{net::UdpSocket, time::sleep};

#[no_mangle]
pub extern "C" fn slo_main() {
    println!("Starting LED blinker");

    // Initialize Tokio
    let rt = tokio::runtime::Builder::new_current_thread()
        .thread_name("tokio worker")
        .max_blocking_threads(20)
        .enable_all()
        .build()
        .unwrap();

    // Start the main async task
    rt.block_on(async_main());
}

async fn async_main() -> ! {
    // Start a UDP server
    let listener = UdpSocket::bind("0.0.0.0:52000")
        .await
        .expect("failed to bind to '0.0.0.0:52000'");
    let mut recv_buf = [0u8; 256];

    // Start a blinker
    let blink_fut = blink(Duration::from_micros(200_000)).fuse();
    tokio::pin!(blink_fut); // blink_fut: Pin<&mut _>

    loop {
        // Poll all given futures and execute the corresponding arm when one of them completes
        futures::select! {
            // If we receive a new blinking interval through the UDP socket, restart the blinker
            // using the new interval
            result = listener.recv(&mut recv_buf).fuse() => match result {
                Ok(recv_len) => {
                    // Parse the received packet
                    let new_interval_str = String::from_utf8_lossy(&recv_buf[..recv_len]);
                    let new_interval: Option<u32> = new_interval_str.trim().parse().ok();

                    if let Some(new_interval) = new_interval {
                        eprintln!("info: changing the interval to {new_interval}μs");

                        // Create a new blinker
                        let new_blink_fut = blink(Duration::from_micros(new_interval.into())).fuse();

                        // Replace the old blinker with the new one
                        blink_fut.set(new_blink_fut);
                    } else {
                        eprintln!("error: invalid interval '{new_interval_str}'");
                    }
                }
                Err(e) => eprintln!("error: recv failed: {e}"),
            },

            // Allow the blinker to make progress
            _ = blink_fut => unreachable!(),
        }
    }
}

async fn blink(interval: Duration) -> ! {
    // Configure the LED port
    green_led::init();

    loop {
        // Turn on the LED
        green_led::update(true);
        sleep(interval).await;

        // Turn off the LED
        green_led::update(false);
        sleep(interval).await;
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

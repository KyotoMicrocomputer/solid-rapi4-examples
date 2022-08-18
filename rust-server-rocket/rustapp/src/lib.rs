#![deny(unsafe_op_in_unsafe_fn)]
#![feature(decl_macro)]

#[cfg(target_os = "solid_asp3")]
mod stubs;

/// Called by the main task entry point defined in `main.cpp`
#[no_mangle]
extern "C" fn rust_entry() {
    std::panic::resume_unwind(
        std::thread::Builder::new()
            // Rocket's startup code is very stack-heavy
            .stack_size(256 * 1024)
            .spawn(rust_entry_inner)
            .expect("failed to spawn a thread")
            .join()
            .err()
            .unwrap(),
    );
}

fn rust_entry_inner() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Initialize Tokio
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(solid::abi::SOLID_CORE_MAX)
        .thread_name("tokio worker")
        .on_thread_start(|| {
            #[cfg(target_os = "solid_asp3")]
            {
                // Distribute the worker threads across all processors
                use std::sync::atomic::{AtomicUsize, Ordering};
                static TID: AtomicUsize = AtomicUsize::new(0);
                let thread_index = TID.fetch_add(1, Ordering::Relaxed);
                let i = thread_index % solid::abi::SOLID_CORE_MAX;
                itron::task::current()
                    .unwrap()
                    .as_ref()
                    .migrate(itron::processor::Processor::from_raw(i as i32 + 1).unwrap())
                    .unwrap();
            }
        })
        .max_blocking_threads(20)
        .enable_all()
        .build()
        .unwrap();

    // Start HTTP server
    rt.block_on(server_loop());
}

// ----------------------------------------------------------------------------
//                                HTTP Server
// ----------------------------------------------------------------------------

#[macro_use]
extern crate rocket;

/// Start an HTTP server on the current async task. This async function will never complete.
async fn server_loop() -> ! {
    let config = rocket::Config {
        address: [0, 0, 0, 0].into(),
        port: 8080,
        temp_dir: r"\OSCOM_FS\tmp".into(),
        ..Default::default()
    };

    let _rocket = rocket::custom(&config)
        .mount("/hello", routes![world])
        .launch()
        .await
        .expect("web server failed to start");

    panic!("web server exited unexpectedly");
}

// ----------------------------------------------------------------------------
//                              Request Handlers
// ----------------------------------------------------------------------------

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

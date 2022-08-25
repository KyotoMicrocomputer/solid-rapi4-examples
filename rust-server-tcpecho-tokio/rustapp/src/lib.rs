#![feature(solid_ext)] // std::os::solid::prelude::AsRawFd
use futures::FutureExt;
use std::{future::Future, io, os::solid::prelude::AsRawFd, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Semaphore,
};

const NUM_WORKERS: usize = 32;

/// The root task entry point.
#[no_mangle]
pub extern "C" fn slo_main() {
    // Register a logger for use by `log::*` macros
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    // Initialize Tokio runtime using the current-thread executor (all async tasks
    // run on the current thread)
    let rt = tokio::runtime::Builder::new_current_thread()
        .thread_name("tokio worker")
        .max_blocking_threads(20)
        .enable_all()
        .build()
        .unwrap();

    // Start the main async task on the current thread
    rt.block_on(async_main());
}

/// The main async task.
async fn async_main() {
    // Create a semaphore to limit the maximum concurrency
    let semaphore = Arc::new(Semaphore::new(NUM_WORKERS));

    // Create an accepting socket
    let listener = TcpListener::bind(("0.0.0.0", 7777)).await.expect("bind");
    log::info!(
        "Starting TCP echo server on {}",
        listener.local_addr().expect("local_addr")
    );

    let mut connection = Connection(0);

    // Accept clients
    loop {
        // Get a permit to spawn a new
        let semaphore_guard = {
            let acquire_fut = Arc::clone(&semaphore).acquire_owned();
            tokio::pin!(acquire_fut);

            // If a permit is already available, this future will resolve immediately.
            if let Some(x) = (&mut acquire_fut).now_or_never() {
                x
            } else {
                // Otherwise, we must wait until a permit becomes available.
                log::debug!("Waiting for a permit to become available");
                acquire_fut.await
            }
        }
        .expect("semaphore closed");

        // Accept a new client
        let (client_fd, _) = match listener.accept().await {
            Ok(x) => x,
            Err(e) => {
                log::warn!("Failed to accept a client: {e:?}");

                // An accept failure is mostly non-fatal, so continue
                continue;
            }
        };

        connection.0 += 1;
        let connection = connection.clone();

        // Make sure `semaphore_guard` is dropped last
        let mut worker_input = (client_fd, semaphore_guard);

        // Spawn a worker
        tokio::spawn(async move {
            let (client_fd, _) = &mut worker_input;

            // Serve the client
            let client_fd_raw = client_fd.as_raw_fd(); // for logging
            log::info!("{connection:?}: Serving client FD {client_fd_raw}");
            if let Err(e) = serve_client(client_fd, &mut vec![0u8; 4096]).await {
                log::info!(
                    "{connection:?}: Finished serving client FD {client_fd_raw} with error: {e:?}"
                );
            } else {
                log::info!(
                    "{connection:?}: Finished serving client FD {client_fd_raw} successfully"
                );
            }
        });
    }
}

/// Identifies a connection. Only used for diagnostic purposes.
#[derive(Debug, Clone)]
struct Connection(u64);

async fn serve_client(client_fd: &mut TcpStream, buffer: &mut [u8]) -> io::Result<()> {
    let timeout = Duration::from_secs(30);

    loop {
        // Read data from the socket
        let num_read_bytes = with_timeout(timeout, client_fd.read(buffer)).await?;
        if num_read_bytes == 0 {
            with_timeout(timeout, client_fd.shutdown()).await?;
            break;
        }

        // Write back the data to the socket
        match with_timeout(timeout, client_fd.write_all(&buffer[..num_read_bytes])).await {
            Err(e) if e.kind() == io::ErrorKind::WriteZero => {
                break;
            }
            result => result?,
        }
    }

    Ok(())
}

/// Wrap a given `Future` to set a timeout duration. On timeout, the wrapped `Future`
/// will be cancelled, and the returned `Future` will resolve to an `Err(_)`.
async fn with_timeout<F, T>(duration: Duration, f: F) -> io::Result<T>
where
    F: Future<Output = io::Result<T>>,
{
    tokio::time::timeout(duration, f)
        .await
        .unwrap_or_else(|tokio::time::error::Elapsed { .. }| Err(io::ErrorKind::TimedOut.into()))
}

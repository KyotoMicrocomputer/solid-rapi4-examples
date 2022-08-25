#![feature(let_else)] // let pattern = ... else { ... };
#![feature(solid_ext)] // std::os::solid::prelude::AsRawFd
use std::{
    io::{self, prelude::*},
    net::{Shutdown, TcpListener, TcpStream},
    os::solid::prelude::AsRawFd,
    sync::mpsc,
    time::Duration,
};

const NUM_WORKERS: usize = 8;

/// The root task entry point.
#[no_mangle]
pub extern "C" fn slo_main() {
    // Register a logger for use by `log::*` macros
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let (send_job_ingress, recv_job_ingress) = mpsc::sync_channel(NUM_WORKERS);

    // Create worker threads
    for i in 0..NUM_WORKERS {
        let send_job_ingress = send_job_ingress.clone();
        std::thread::spawn(move || worker_loop(Worker(i), send_job_ingress));
    }
    drop(send_job_ingress);

    // Create an accepting socket
    let listener = TcpListener::bind(("0.0.0.0", 7777)).expect("bind");
    log::info!(
        "Starting TCP echo server on {}",
        listener.local_addr().expect("local_addr")
    );

    // Accept clients
    loop {
        // Find a free worker
        let JobIngress { send_client_fd } =
            recv_job_ingress.recv().expect("worker exited unexpectedly");

        // Accept a new client
        let (client_fd, _) = match listener.accept() {
            Ok(x) => x,
            Err(e) => {
                log::warn!("Failed to accept a client: {e:?}");

                // An accept failure is mostly non-fatal, so continue
                continue;
            }
        };

        // Pass the client FD to the worker
        send_client_fd.send(client_fd).unwrap();
    }
}

/// Sent by a worker when it's ready to accept a new request. The acceptor will use it to
/// give the worker a new job.
struct JobIngress {
    send_client_fd: mpsc::SyncSender<TcpStream>,
}

/// Identifies a worker. Only used for diagnostic purposes.
#[derive(Debug, Copy, Clone)]
struct Worker(usize);

fn worker_loop(worker: Worker, send_job_ingress: mpsc::SyncSender<JobIngress>) {
    let mut buf = [0u8; 4096];

    loop {
        // Get a new job by sending a FD sender to the acceptor
        let (send_client_fd, recv_client_fd) = mpsc::sync_channel(1);
        let _ = send_job_ingress.send(JobIngress { send_client_fd });
        let Ok(client_fd) = recv_client_fd.recv() else {
            // `Err(RecvError)` indicates that `send_client_fd` has been dropped.
            //
            // This implies the `JobIngress` we sent was discarded without being consumed,
            // meaning the acceptor exited for some reason.
            return;
        };

        // Serve the client
        let client_fd_raw = client_fd.as_raw_fd(); // for logging
        log::info!("{worker:?}: Serving client FD {client_fd_raw}");
        if let Err(e) = serve_client(&client_fd, &mut buf) {
            log::info!("{worker:?}: Finished serving client FD {client_fd_raw} with error: {e:?}");
        } else {
            log::info!("{worker:?}: Finished serving client FD {client_fd_raw} successfully");
        }
    }
}

fn serve_client(client_fd: &TcpStream, buffer: &mut [u8]) -> io::Result<()> {
    client_fd.set_write_timeout(Some(Duration::from_secs(30)))?;
    client_fd.set_read_timeout(Some(Duration::from_secs(30)))?;

    loop {
        // Read data from the socket
        let num_read_bytes = (&*client_fd).read(buffer)?;
        if num_read_bytes == 0 {
            client_fd.shutdown(Shutdown::Both)?;
            break;
        }

        // Write back the data to the socket
        match (&*client_fd).write_all(&buffer[..num_read_bytes]) {
            Err(e) if e.kind() == io::ErrorKind::WriteZero => {
                break;
            }
            result => result?,
        }
    }

    Ok(())
}

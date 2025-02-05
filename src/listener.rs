use std::net::UdpSocket;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct Listener {
    running: Arc<AtomicBool>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Listener {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn listen(&mut self) -> Result<(), std::io::Error> {
        self.running.store(true, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let thread_handle = thread::spawn(move || {
            let socket = match UdpSocket::bind("0.0.0.0:5000") {
                Ok(socket) => socket,
                Err(e) => {
                    eprintln!("Failed to bind socket: {}", e);
                    return;
                }
            };

            let mut buf = [0; 1024];
            println!("Listening for UDP messages on port 5000...");

            while running.load(Ordering::SeqCst) {
                match socket.recv_from(&mut buf) {
                    Ok((amt, src)) => {
                        println!("Received '{}' from {}", String::from_utf8_lossy(&buf[..amt]), src);
                    }
                    Err(e) => {
                        eprintln!("UDP Listener Error: {}", e);
                        break; // Exit the loop on error
                    }
                }
            }
        });

        self.thread_handle = Some(thread_handle);
        println!("Listener started...");
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), std::io::Error> {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            handle.join().expect("Listener thread panicked");
        }
        println!("Listener thread shutting down...");
        Ok(())
    }
}

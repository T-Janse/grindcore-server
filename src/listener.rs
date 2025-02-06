use std::net::UdpSocket;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

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
                    running.store(false, Ordering::SeqCst);
                    return;
                }
            };

            socket.set_read_timeout(Some(Duration::from_secs(1)))
                .expect("Failed to set read timeout");
            let mut buf = [0; 1024];
            println!("Listening for UDP messages on port 5000...");
            while running.load(Ordering::SeqCst) {
                 match socket.recv_from(&mut buf) {
                    Ok((amt, src)) => {
                        println!("Received '{}' from {}", String::from_utf8_lossy(&buf[..amt]), src);
                    }
                     Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                         // Timeout occurred, check the running flag again
                         continue;
                     }
                     Err(e) => {
                         running.store(false, Ordering::SeqCst);
                         eprintln!("UDP Listener Error: {}", e);
                         break;
                     }
                }
            }
        });
        if(!self.running.load(Ordering::SeqCst)) {
            self.stop().expect("Failed to close listener");
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), std::io::Error> {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.thread_handle.take() {
            handle.join().unwrap();
        }
        println!("Listener thread shutting down...");
        Ok(())
    }
}

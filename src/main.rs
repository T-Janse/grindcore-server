mod listener;
use listener::Listener;
use std::{io, thread};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    let mut listener = Listener::new();
        listener.listen().expect("Failed to launch websocket");
        sleep(Duration::new(10, 0));
        listener.stop().expect("Failed to stop websocket");
        Ok(())
}

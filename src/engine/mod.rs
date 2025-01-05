pub mod handler;

use std::{io::{Error, Read}, net::TcpListener};

use handler::Handler;

pub struct Engine<T: Handler> {
    tcp_payload_handler: T,
    started: bool,
    port: u16,
}

impl<T> Engine<T> where T: Handler {
    pub fn new(tcp_payload_handler: T, port: u16) -> Engine<T>{
        Self {
            tcp_payload_handler,
            started: false,
            port: if port <= 0 { 8080 } else { port },
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&address)?;

        self.started = true;
        println!("Server started on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(mut s) => {
                    let mut buf: [u8; 1024] = [0; 1024];
                    _ = s.read(&mut buf);
                    self.tcp_payload_handler.handle_payload(&mut buf);
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}

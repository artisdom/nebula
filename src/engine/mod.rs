mod handler;

use std::{io::Error, net::TcpListener};

pub struct Server {
    container_path: String,
    started: bool,
    port: u16,
}

impl Server {
    pub fn new(container_path: String, port: u16) -> Result<Self, String> {
        if !std::path::Path::new(&container_path).exists() {
          return Err(format!("Path '{}' does not exist", container_path));
        }

        Ok(Self {
            container_path,
            started: false,
            port: if port <= 0 { 8080 } else { port },
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&address)?;

        self.started = true;
        println!("Server started on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                  let mut mut_stream = s.try_clone().expect("Clone has failed");
                  drop(s);
                  self.handle_stream(&mut mut_stream);
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }
}

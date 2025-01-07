mod utils;

use std::{io::Write, path::Path};
use crate::mult_thread::ProtocolHandler;

pub struct HttpProtocolHandler {
    default_container: String
}

impl ProtocolHandler for HttpProtocolHandler {
    fn handle(&mut self, tcp_stream: &mut std::net::TcpStream) {
        let http_header = match utils::decompress(tcp_stream) {
            Ok(s) => s,
            Err(_) => panic!("") // Handle with non utf, fails in bufreader etc.
        };

        let req_parts: Vec<&str> = http_header.split_whitespace().collect();
        let mut req_path = req_parts[1];

        if req_path == "/" {
            req_path = "/index.html";
        }

        if self.default_container.ends_with("/") {
            self.default_container.remove(self.default_container.len() - 1);
        }

        if !req_path.contains(".html") && !req_path.contains(".css") && !req_path.contains(".js") {
            // This isn't the better way to do the things.
            panic!("The nebula server only supports JavaScript, Html and Css files: {}", req_path);
        }

        let str_path = format!("{}{}", self.default_container, req_path);
        let file_path = Path::new(str_path.as_str());
        let file_path = file_path.to_str().unwrap().to_string();

        let result = match utils::read_file_content_as_string(&file_path) {
            Ok(x) => {
                let path = Path::new(file_path.as_str());
                let extension = path.extension().unwrap().to_str().unwrap().to_string();
                utils::mount(&extension, &x)
            },
            Err(_) => {
                // if we get here, means the file couldn't be found,
                // so we must return appropriated response
                utils::mount_not_founded()
            }
        };


        _ = tcp_stream.write(result.as_bytes());
        _ = tcp_stream.flush();
    }
}

impl HttpProtocolHandler {
    pub fn new(container_path: String) -> impl ProtocolHandler {
        HttpProtocolHandler {
            default_container: container_path
        }
    }
}
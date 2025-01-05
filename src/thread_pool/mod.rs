use std::{io::Write, net::TcpStream, sync::{Arc, Mutex}, thread};
use crate::engine::handler::Handler;

const DEFAULT_HTTP_RESP_MSG: &str = "
    HTTP/1.1 400 OK
    Content-Type: text/html; charset=UTF-8
    
    <body>
    </body>
";

pub struct ThreadHandler;

fn __handle_payload(payload: TcpStream) {
    let payload = Arc::new(Mutex::new(payload));
    thread::spawn({
        let payload = Arc::clone(&payload);
        move || {
            let mut payload = payload.lock().unwrap();
            _ = payload.write(DEFAULT_HTTP_RESP_MSG.as_bytes());
        }
    });
}

impl Handler for ThreadHandler {
    fn handle_payload(&self, payload: &mut TcpStream) {
        __handle_payload(payload.try_clone().unwrap());
    }
}

impl ThreadHandler {
    pub fn new() -> impl Handler {
        ThreadHandler
    }
}

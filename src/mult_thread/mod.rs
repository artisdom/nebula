use crate::engine::handler::Handler;
use std::{sync::{Arc, Mutex}, thread};
use std::net::TcpStream;

pub struct ThreadHandler<T: ProtocolHandler + Send> { 
    protocol_handler: Arc<Mutex<T>>,
}

impl<T> Handler for ThreadHandler<T>
where 
    T: ProtocolHandler + Send + 'static
{
    fn handle_payload(&self, payload: TcpStream) {
        let mut cloned_payload = payload.try_clone().unwrap();
        let protocol_handler = self.protocol_handler.clone(); 

        thread::spawn(move || {
            let mut handler = protocol_handler.lock().unwrap(); 

            handler.handle(&mut cloned_payload);
        });
    }
}

impl<T> ThreadHandler<T>
where
    T: ProtocolHandler + Send + 'static
{
    pub fn new(protocol_handler: T) -> ThreadHandler<T> {
        ThreadHandler {
            protocol_handler: Arc::new(Mutex::new(protocol_handler)),  // Wrap the handler in Arc<Mutex>
        }
    }
}


pub trait ProtocolHandler {
    fn handle(&mut self, tcp_stream: &mut TcpStream);
}

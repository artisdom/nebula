use std::net::TcpStream;

pub trait Handler {
    fn handle_payload(&self, payload: &mut TcpStream);
}
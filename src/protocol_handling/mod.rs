use std::net::TcpStream;

pub trait ProtocolHandler {
    fn handle(&mut self, tcp_stream: &mut TcpStream);
}

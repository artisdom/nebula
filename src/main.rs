use nebula::{engine::Engine, http_protocol_handler::HttpProtocolHandler, mult_thread::ThreadHandler};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let path = args.get(1).expect("No container to server specified. Use cargo run /path/to/container");
    Engine::new(ThreadHandler::new(HttpProtocolHandler::new(path.clone())), 8080).start().unwrap();
}
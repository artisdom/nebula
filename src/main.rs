use nebula::{engine::Engine, thread_pool::ThreadHandler};

fn main() {
    Engine::new(&ThreadHandler::new(), 8080).start().unwrap();
}
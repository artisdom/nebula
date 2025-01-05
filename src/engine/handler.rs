pub trait Handler {
    fn handle_payload(&mut self, buf: &[u8]);
}
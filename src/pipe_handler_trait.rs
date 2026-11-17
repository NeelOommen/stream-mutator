pub trait PipeHandler: Send + Sync {
    fn consume_byte(&mut self, input: u8) -> Vec<u8>;
}
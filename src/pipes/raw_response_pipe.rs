use crate::pipe_handler_trait::PipeHandler;

pub struct RawLoggingPipe;

impl RawLoggingPipe {
    pub fn new() -> RawLoggingPipe {
        RawLoggingPipe
    }
}

impl PipeHandler for RawLoggingPipe {
    fn consume_byte(&mut self, input: u8) -> Vec<u8> {
        print!("{}", input);
        vec![input]
    }
}
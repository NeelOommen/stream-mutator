use crate::pipe_handler_trait::PipeHandler;

pub struct DefaultStateMachine;

impl DefaultStateMachine {
    pub fn new() -> DefaultStateMachine {
        DefaultStateMachine
    }
}

impl PipeHandler for DefaultStateMachine {
    fn consume_byte(&mut self, input: u8) -> Vec<u8> {
        vec![input]
    }
}
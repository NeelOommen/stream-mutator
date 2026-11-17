use crate::state_machine_trait::StateMachine;

pub struct DefaultStateMachine;

impl StateMachine for DefaultStateMachine {
    fn consume_byte(&mut self, input: u8) -> Vec<u8> {
        vec![input]
    }
}
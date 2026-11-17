use crate::pipe_handler_trait::PipeHandler;

pub struct ByteMetricPipeHandler {
    passed_byte_count: u64
}

impl ByteMetricPipeHandler {
    pub fn new() -> ByteMetricPipeHandler {
        ByteMetricPipeHandler{
            passed_byte_count: 0
        }
    }
}

impl PipeHandler for ByteMetricPipeHandler {
    fn consume_byte(&mut self, input: u8) -> Vec<u8> {
        self.passed_byte_count += 1;
        vec![input]
    }
}
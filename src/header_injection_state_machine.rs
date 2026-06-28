//consuming state trait
trait TransitionState{
    fn transition(&self, input: u8) -> TransitionResult;
}

struct TransitionResult{
    output: Vec<u8>,
    next_state: States
}

//state machine
enum States{
    PEOH(PreEndOfHeaders),
    FCR(FirstCR),
    FLF(FirstLF),
    SCR(SecondCR),
    AEOH(PostHeaders)
}

impl TransitionState for States{
    fn transition(&self, input: u8) -> TransitionResult {
        match self{
            States::PEOH(s) => s.transition(input),
            States::FCR(s) => s.transition(input),
            States::FLF(s) => s.transition(input),
            States::SCR(s) => s.transition(input),
            States::AEOH(s) => s.transition(input),
        }
    }
}


pub struct HttpMachine {
    state: States,
}

impl HttpMachine {
    pub fn new() -> HttpMachine {
        HttpMachine{
            state: States::PEOH(PreEndOfHeaders{}),
        }
    }

    pub fn consume_byte(&mut self, input: u8) -> Vec<u8> {
        let transition_result = self.state.transition(input);
        self.state = transition_result.next_state;
        transition_result.output
    }
}

//states
struct PreEndOfHeaders;
impl TransitionState for PreEndOfHeaders{
    fn transition(&self, input: u8) -> TransitionResult {
        match &input{
            13 => TransitionResult{output: vec![], next_state: States::FCR(FirstCR{})}, //hold the CR
            _ => TransitionResult{output: vec![input], next_state: States::PEOH(PreEndOfHeaders{})},
        }
    }
}

struct FirstCR;
impl TransitionState for FirstCR{
    fn transition(&self, input: u8) -> TransitionResult {
        match &input{
            10 => TransitionResult{output: vec![], next_state: States::FLF(FirstLF{})}, //hold the LF
            _ => TransitionResult{output: vec![13, input], next_state: States::PEOH(PreEndOfHeaders{})},
        }
    }
}

struct FirstLF;
impl TransitionState for FirstLF{
    fn transition(&self, input: u8) -> TransitionResult {
        match &input{
            13 => TransitionResult{output: vec![], next_state: States::SCR(SecondCR{})}, //hold the 2nd CR
            _ => TransitionResult{output: vec![13, 10, input], next_state: States::PEOH(PreEndOfHeaders{})},
        }
    }
}

struct SecondCR;
impl SecondCR{
    fn injectable_header(&self) -> Vec<u8> {
        let header_string = String::from("stream-mutator: this-is-new");
        header_string.into_bytes()
    }
}
impl TransitionState for SecondCR{
    fn transition(&self, input: u8) -> TransitionResult {
        match &input{
            10 => {
                let mut injectable_header = self.injectable_header();
                injectable_header.splice(0..0, [13,10]); //end the previous header
                injectable_header.extend_from_slice(&[13,10,13,10]);
                //^^^CRLF for injected header + CRLF to end header section
                TransitionResult{output: injectable_header, next_state: States::AEOH(PostHeaders{})}
            }, //push out the extra header and the CRLFCRLF
            _ => TransitionResult{output: vec![13, 10, 13, input], next_state: States::PEOH(PreEndOfHeaders{})},
        }
    }
}

struct PostHeaders;
impl TransitionState for PostHeaders{
    fn transition(&self, input: u8) -> TransitionResult {
        //just keep dumping the input as is, no modification
        TransitionResult{output: vec![input], next_state: States::AEOH(PostHeaders{})}
    }
}
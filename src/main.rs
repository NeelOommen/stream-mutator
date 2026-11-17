mod pipe_handler;
mod pipe_mode;
mod default_metric_pipe_handler;
mod raw_request_logging_pipe_handler;
pub mod state_machines;
pub mod state_machine_trait;

use std::io::ErrorKind;
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::pipe_mode::PipeMode;
use crate::pipe_mode::PipeMode::*;
use crate::state_machine_trait::StateMachine;
use crate::state_machines::default_state_machine::DefaultStateMachine;
use crate::state_machines::header_injection_state_machine::HeaderInjectionStateMachine;

#[tokio::main]
async fn main() {
    let cts_mode = HeaderInjectionMode;
    let stc_mode = HeaderInjectionMode;

    let listener = TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Could not listen for connections");

    loop{
        let (stream, _) = listener.accept().await.unwrap();

        //start a thread to handle this stream
        tokio::spawn(async move {
            handle_connection(cts_mode, stc_mode, stream).await;
        });
    }
}

async fn handle_connection(cts_mode: PipeMode, stc_mode: PipeMode, stream: TcpStream) {
    let downstream_target: String = String::from("127.0.0.1:8080");
    let upstream = TcpStream::connect(downstream_target).await.unwrap();

    let (client_reader, client_writer) = stream.into_split();
    let (server_reader, server_writer) = upstream.into_split();

    let _client_to_server = tokio::spawn(async move {
        state_machine_pipe_handler(client_reader, server_writer, cts_mode).await;
    });

    let _server_to_client = tokio::spawn(async move {
        state_machine_pipe_handler(server_reader, client_writer, stc_mode).await;
    });
}

async fn state_machine_pipe_handler(reader: OwnedReadHalf, writer: OwnedWriteHalf, mode: PipeMode) {
    let mut machine = get_required_state_machine(mode);
    let mut buf: [u8; 256] = [0; 256];
    while reader.readable().await.is_ok() {
        let read_size = reader.try_read(&mut buf);
        match read_size {
            Ok(x) => {
                if x > 0 {
                    let output = process_buffer(&mut machine, &buf, x);
                    let write_size = writer.try_write(output.as_slice());
                    match write_size {
                        Err(we) => {
                            eprintln!("Error writing to server: {:?}", we);
                        }
                        _ => {}
                    }
                }
                else {
                    break;
                }
            }
            Err(e) => {
                println!("Error reading from source: {:?}", e);
                if e.kind() != ErrorKind::WouldBlock {
                    break;
                }
            }
        }
    }
}

fn process_buffer(machine: &mut Box<dyn StateMachine>, buffer: &[u8; 256], read_size: usize) -> Vec<u8> {
    let mut i: usize = 0;
    let mut output: Vec<u8> = Vec::with_capacity(read_size); //minimum required capacity of read size
    while i < read_size {
        output.append(&mut machine.consume_byte(buffer[i]));
        i+=1;
    }
    output
}

fn get_required_state_machine(mode: PipeMode) -> Box<dyn StateMachine> {
    match mode {
        DefaultMode | DefaultMetricMode | RawResponseLoggingMode => Box::new(DefaultStateMachine),
        HeaderInjectionMode => Box::new(HeaderInjectionStateMachine::new()),
    }
}
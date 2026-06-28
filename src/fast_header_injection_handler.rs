use std::io::{Error, ErrorKind};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use crate::header_injection_state_machine::HttpMachine;
use crate::pipe_handler::PipeHandler;

pub struct FastHeaderInjectingPipeHandler;

impl PipeHandler for FastHeaderInjectingPipeHandler {
    async fn handle_connection(stream: TcpStream, downstream_target: String) -> Result<(), Error> {
        let upstream = TcpStream::connect(downstream_target).await.unwrap();

        let (client_reader, client_writer) = stream.into_split();
        let (server_reader, server_writer) = upstream.into_split();

        let client_to_server = tokio::spawn(async move {
            Self::state_machine_pipe_handler(client_reader, server_writer).await;
        });

        let server_to_client = tokio::spawn(async move {
            Self::state_machine_pipe_handler(server_reader, client_writer).await;
        });

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}

impl FastHeaderInjectingPipeHandler {
    async fn state_machine_pipe_handler(reader: OwnedReadHalf, writer: OwnedWriteHalf) {
        let mut machine = HttpMachine::new();
        let mut buf: [u8; 256] = [0; 256];
        while reader.readable().await.is_ok() {
            let read_size = reader.try_read(&mut buf);
            match read_size {
                Ok(x) => {
                    if x > 0 {
                        let output = Self::process_buffer(&mut machine, &buf, x);
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

    fn process_buffer(machine: &mut HttpMachine, buffer: &[u8; 256], read_size: usize) -> Vec<u8> {
        let mut i: usize = 0;
        let mut output: Vec<u8> = Vec::with_capacity(read_size); //minimum required capacity of read size
        while i < read_size {
            output.append(&mut machine.consume_byte(buffer[i]));
            i+=1;
        }
        output
    }
}
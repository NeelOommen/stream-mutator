use crate::pipe_handler::PipeHandler;
use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct RawRequestLoggingPipeHandler;

impl RawRequestLoggingPipeHandler {
    async fn generic_pipe_handler(reader: OwnedReadHalf, writer: OwnedWriteHalf, name: &str) {
        let mut buf: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        while reader.readable().await.is_ok() {
            let read_size = reader.try_read(&mut buf);
            match read_size {
                Ok(x) => {
                    if x > 0 {
                        println!("{}: Data read from client: {:?}", name, String::from_utf8_lossy(&buf[..x]));
                        let write_size = writer.try_write(&buf[..x]);
                        match write_size {
                            Ok(ws) => {
                                println!("{}: Successfully wrote {} bytes to server", name, ws);
                            }
                            Err(we) => {
                                eprintln!("{}: Error writing to server: {:?}", name, we);
                            }
                        }
                    }
                    else {
                        break;
                    }
                }
                Err(e) => {
                    println!("{}: Error reading from client: {:?}", name, e);
                    if e.kind() != ErrorKind::WouldBlock {
                        break;
                    }
                }
            }
        }
    }
}

impl PipeHandler for RawRequestLoggingPipeHandler {
    async fn handle_connection(stream: TcpStream, downstream_target: String) -> Result<(), Error> {
        let upstream = TcpStream::connect(downstream_target).await.unwrap();

        let (client_reader, client_writer) = stream.into_split();
        let (server_reader, server_writer) = upstream.into_split();

        let client_to_server = tokio::spawn(async move {
            Self::generic_pipe_handler(client_reader, server_writer, "C->S").await;
        });

        let server_to_client = tokio::spawn(async move {
            Self::generic_pipe_handler(server_reader, client_writer, "S->C").await;
        });

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}

use crate::pipe_handler::PipeHandler;
use std::io::Error;
use tokio::net::TcpStream;

pub(crate) struct DefaultMetricPipeHandler;

impl PipeHandler for DefaultMetricPipeHandler {
    async fn handle_connection(stream: TcpStream, downstream_target: String) -> Result<(), Error> {
        let upstream = TcpStream::connect(downstream_target).await.unwrap();

        let (mut client_reader, mut client_writer) = stream.into_split();
        let (mut server_reader, mut server_writer) = upstream.into_split();

        let s1 = tokio::spawn(async move {
            let r = tokio::io::copy(&mut client_reader, &mut server_writer).await;
            println!("Client read: {:?}", r);
            r
        });

        let s2 = tokio::spawn(async move {
            let r = tokio::io::copy(&mut server_reader, &mut client_writer).await;
            println!("Server read: {:?}", r);
            r
        });

        let _ = tokio::try_join!(
            s1, s2, //pass server response to client
        )?;

        Ok(())
    }
}

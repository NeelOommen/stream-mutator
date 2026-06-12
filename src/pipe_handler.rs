use tokio::net::TcpStream;

pub trait PipeHandler {
    async fn handle_connection(stream: TcpStream, downstream_target: String) -> Result<(),std::io::Error>;
}
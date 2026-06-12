mod pipe_handler;
mod default_pipe_handler;
mod pipe_mode;
mod default_metric_pipe_handler;

use tokio::net::{TcpListener, TcpStream};
use crate::default_metric_pipe_handler::DefaultMetricPipeHandler;
use crate::pipe_handler::PipeHandler;
use crate::pipe_mode::PipeMode;
use crate::default_pipe_handler::DefaultPipeHandler;
use crate::pipe_mode::PipeMode::*;

#[tokio::main]
async fn main() {
    let mode = DefaultMetricMode;

    let listener = TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Could not listen for connections");

    loop{
        let (stream, socket) = listener.accept().await.unwrap();

        let _ = handle_connection(mode, stream).await;
    }
}

async fn handle_connection(mode: PipeMode, stream: TcpStream) {
    let target: String = String::from("127.0.0.1:8080");
    let task = tokio::spawn(async move {
        let result = match(mode){
            DefaultMode =>  DefaultPipeHandler::handle_connection(stream, target).await,
            DefaultMetricMode =>  DefaultMetricPipeHandler::handle_connection(stream, target).await,
        };

        if let Err(e) = result {
            eprintln!("Error: {}", e);
        }
    });
    let _ = task.await;
}
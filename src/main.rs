use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081")
        .await
        .expect("Could not listen for connections");

    loop{
        let (stream, socket) = listener.accept().await.unwrap();
        let result = handle_connection(stream);

        tokio::spawn(async move {
            if let Err(e) = result.await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(),std::io::Error> {
    let upstream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

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
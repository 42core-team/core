use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

async fn handle_client(mut stream: tokio::net::TcpStream, sender: mpsc::Sender<tokio::net::TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    let mut sender = sender;

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break; // Connection closed
        }

        // Convert the received data to a string
        let received_data = String::from_utf8_lossy(&buffer[0..n]);
        println!("Received data: {}", received_data);

        // Echo the data back to the client
        stream.write_all(&buffer[0..n]).await?;
        println!("Sent data back to client");
    }

    Ok(())
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
	let listener = TcpListener::bind("127.0.0.1:3500").await?;
    println!("Server listening on 127.0.0.1:3500");

    let (sender, mut receiver) = mpsc::channel(32);

    while let Ok((stream, _)) = listener.accept().await {
        let sender = sender.clone(); // Clone the sender for each client

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, sender).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }

    while let Some(stream) = receiver.recv().await {
        let sender = sender.clone(); // Clone the sender for each client

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, sender).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }

    Ok(())
}

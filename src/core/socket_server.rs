use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;

async fn handle_client(mut stream: tokio::net::TcpStream, sender: mpsc::Sender<tokio::net::TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer: [u8; 1024] = [0; 1024];
    let mut _sender = sender;
    let json_data: &str = r#"
    {
        "map":{
                "width": 100000,
                "height": 100000,
                "teams": [
                    { "id": 1, "name": "Team 1" },
                    { "id": 2, "name": "Team 2" }
                ],
                "entities": [
                    { "Core": { "team_id": 2, "x": 4, "y": 5 } },
                    { "Core": { "team_id": 1, "x": 5, "y": 6 } },
                    { "Unit": { "Warrior": {"team_id": 1, "x": 5, "y": 6 } } },
                    { "Unit": { "Warrior": {"team_id": 2, "x": 6, "y": 7 } } },
                    { "Unit": { "Worker": {"team_id": 1, "x": 7, "y": 6 } } },
                    { "Unit": { "Worker": {"team_id": 2, "x": 8, "y": 7 } } },
                    { "Resource": { "value": 1, "x": 9, "y": 6 } }
                ]
            }
    }
"#;

    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break; // Connection closed
        }

        // Convert the received data to a string
        let received_data = String::from_utf8_lossy(&buffer[0..n]);
        println!("Received data: {}", received_data);

        // Echo the data back to the client
        stream.write_all(json_data.as_bytes()).await?;
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

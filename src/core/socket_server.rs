// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::sync::mpsc;

// async fn handle_client(mut stream: tokio::net::TcpStream, sender: mpsc::Sender<tokio::net::TcpStream>) -> Result<(), Box<dyn std::error::Error>> {
//     let mut buffer: [u8; 1024] = [0; 1024];
//     let mut _sender = sender;
//     let json_data: &str = r#"
//     {
// 		"status": 0,
// 		"entities": [
// 			{
// 				"Core": {
// 					"id": 0,
// 					"team_id": 0,
// 					"x": 5000,
// 					"y": 6000,
// 					"hp": 10000
// 				}
// 			},
// 			{
// 				"Resource": {
// 					"id": 1,
// 					"value": 300,
// 					"x": 3000,
// 					"y": 2000,
// 					"hp": 4000
// 				}
// 			}
// 		],
// 		"units": [
// 			{
// 				"id": 2,
// 				"type_id": 0,
// 				"hp": 3000,
// 				"x": 8000,
// 				"y": 6000,
// 				"team_id": 1
// 			}
// 		]
// 	}
// "#;

//     while let Ok(n) = stream.read(&mut buffer).await {
//         if n == 0 {
//             break; // Connection closed
//         }

//         // Convert the received data to a string
//         let received_data = String::from_utf8_lossy(&buffer[0..n]);
//         println!("Received data: {}", received_data);

//         // Echo the data back to the client
//         stream.write_all(json_data.as_bytes()).await?;
//         println!("Sent data back to client");
//     }

//     Ok(())
// }

// pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
// 	let listener = TcpListener::bind("127.0.0.1:4242").await?;
//     println!("Server listening on 127.0.0.1:4242");

//     let (sender, mut receiver) = mpsc::channel(32);

//     while let Ok((stream, _)) = listener.accept().await {
//         let sender = sender.clone(); // Clone the sender for each client

//         tokio::spawn(async move {
//             if let Err(e) = handle_client(stream, sender).await {
//                 eprintln!("Error handling client: {}", e);
//             }
//         });
//     }

//     while let Some(stream) = receiver.recv().await {
//         let sender = sender.clone(); // Clone the sender for each client

//         tokio::spawn(async move {
//             if let Err(e) = handle_client(stream, sender).await {
//                 eprintln!("Error handling client: {}", e);
//             }
//         });
//     }

//     Ok(())
// }

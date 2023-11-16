// use tokio::net::TcpStream;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use std::str;
// use std::error::Error;

// pub async fn connect(addr: &str) -> Result<TcpStream, Box<dyn Error>> {
// 	let socket = TcpStream::connect(addr).await?;

// 	Ok(socket)
// }

// pub async fn get_response(socket: &mut TcpStream) -> Result<String, Box<dyn Error>> {
//     let mut buffer: [u8; 1024] = [0; 1024];

// 	socket.write("uwu".as_bytes()).await?;
// 	socket.read(&mut buffer).await?;

//     Ok(String::from_utf8_lossy(&buffer).to_string())
// }

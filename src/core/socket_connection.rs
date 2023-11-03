use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // Read data from the client
    match stream.read(&mut buffer) {
        Ok(n) => {
            // Convert the received data to a string
            let received_data = String::from_utf8_lossy(&buffer[0..n]);
            println!("Received data: {}", received_data);

            // Echo the data back to the client
            match stream.write_all(&buffer[0..n]) {
                Ok(_) => {
                    println!("Sent data back to client");
                },
                Err(e) => {
                    eprintln!("Error sending data back to client: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}

fn start_server() {
	let listener = TcpListener::bind("127.0.0.1:3500")?;
    println!("Server listening on 127.0.0.1:3500");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each incoming client
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting client connection: {}", e);
            }
        }
    }
}

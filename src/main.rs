use std::{io::Read, net::{TcpListener, TcpStream}};
fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").expect("Failed to start TCP listener on port 7878");

    println!("Server running on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");
                handle_connection(&mut stream);
            }
            Err(e) => {
                eprintln!("Error connecting to client: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!(
                "Received {} bytes: {}",
                bytes_read,
                String::from_utf8_lossy(&buffer[..bytes_read])
            );
        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}

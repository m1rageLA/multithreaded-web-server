use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
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

    let bytes_read = stream.read(&mut buffer).unwrap_or(0);

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";

    stream
        .write(response.as_bytes())
        .expect("Failed to write response to client");
}

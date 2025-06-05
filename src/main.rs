use std::{
    fs,
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

    let _ = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            0
        }
    };

    let get_root = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| String::from("File not found"));

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response to client");
}

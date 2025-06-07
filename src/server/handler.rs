use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};


pub fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            return;
        }
    };

    let request_text = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => {
            println!("{}", s);
            s
        }
        Err(_) => {
            eprintln!("Request is not valid UTF-8");
            return;
        }
    };

    let request_line = match request_text.lines().next() {
        Some(line) => line,
        None => {
            eprintln!("Request is empty");
            return;
        }
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    let path = if path == "/" {
        "index.html".to_string()
    } else {
        path[1..].to_string()
    };

    let (status_line, filename) = if method == "GET" {
        ("HTTP/1.1 200 OK", path)
    } else {
        ("HTTP/1.1 404 Not Found", "/404.html".to_string())
    };

    let contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", filename);
        String::from("File not found")
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    };
}

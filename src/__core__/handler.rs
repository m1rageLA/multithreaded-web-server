use std::{
    fs,
    io::{Read, Write},
    net::TcpStream,
};

/// Вынесена логика разбора запроса + ответа
pub fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
            return;
        }
    };

    let request = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Request is not valid UTF-8");
            return;
        }
    };
    println!("Request:\n{}", request);

    // простейшая HTTP-логика: только GET + файлы
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/").trim_start_matches('/');

    if method == "GET" {
        // файл по пути или 404.html
        let filename = if path.is_empty() { "index.html" } else { path };
        let (status, content) = match fs::read_to_string(filename) {
            Ok(body) => ("HTTP/1.1 200 OK", body),
            Err(_) => (
                "HTTP/1.1 404 Not Found",
                fs::read_to_string("404.html").unwrap_or_default(),
            ),
        };

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            content.len(),
            content
        );
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to send response: {}", e);
        }
    } else {
        // Method not allowed
        let resp = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
    }
}

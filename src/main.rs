// src/main.rs
mod server;
use std::io::Write;
use server::Server;

fn main() {
    let mut app = Server::new("127.0.0.1:7878".to_string());
    app.route("", |s| {
        let body = "<h1>Welcome home!</h1>";
        let resp = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
        let _ = s.write_all(resp.as_bytes());
    });

    app.route("about", |s| {
        let body = "<h1>About us</h1>";
        let resp = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
        let _ = s.write_all(resp.as_bytes());
    });

    app.run();
}

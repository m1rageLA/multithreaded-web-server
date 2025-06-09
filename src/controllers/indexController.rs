use std::net::TcpStream;
use std::io::Write;

pub fn index_controller() -> impl Fn(&mut TcpStream) + Send + Sync + 'static {
    move |stream| {
        let response = "HTTP/1.1 200 OK\r\n Content-Type: text/html\r\n\r\nHello from controller!";
        stream.write_all(response.as_bytes()).unwrap();
    }
}

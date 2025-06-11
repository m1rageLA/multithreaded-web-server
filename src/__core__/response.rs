use std::{io::Write, net::TcpStream};

use crate::__core__::types::{Request, Response};

impl<'a> Response<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Response {
            stream,
            status: 200,
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn send(&mut self, message: &str) {
        //format can bundle the message
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", message);
        self.stream.write_all(response.as_bytes()).unwrap();
        println!("{}", message);
    }
}

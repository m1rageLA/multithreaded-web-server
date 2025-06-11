use std::net::TcpStream;

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
        println!("{}", message);
    }
}

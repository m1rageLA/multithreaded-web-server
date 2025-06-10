use crate::__core__::types::{Request, Response};
use crate::__core__::{App, Route};
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

impl App {
    pub fn new(address: String) -> Self {
        Self {
            address: address.to_string(),
            routes: Vec::new(),
        }
    }
    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");
        println!("Server is running on {}", self.address);

        let routes: Arc<Vec<Route>> = Arc::new(self.routes);

        for stream in listener.incoming() {
            let routes = Arc::clone(&routes);

            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        let mut buffer = [0; 1024];
                        let n = stream.read(&mut buffer).unwrap_or(0);
                        let req_str = String::from_utf8_lossy(&buffer[..n]);
                        let first = req_str.lines().next().unwrap_or("");
                        let path = first.split_whitespace().nth(1).unwrap_or("/");
                        let path = path.trim_start_matches('/');

                        if let Some(route) = routes.iter().find(|r| r.path == path) {
                            // Clone the stream for the response to avoid mutable aliasing
                            let mut response_stream =
                                stream.try_clone().expect("Failed to clone stream");
                            let mut request_stream =
                                stream.try_clone().expect("Failed to clone stream");
                            let req = Request {
                                stream: &mut request_stream,
                                path: path.to_string(),
                                method: first.split_whitespace().next().unwrap_or("").to_string(),
                            };

                            let res = Response {
                                stream: &mut response_stream,
                                status: 200,
                                headers: vec![],
                                body: vec![],
                            };

                            (route.handler)(req, res);
                        } else {
                            eprintln!("404 Not Found: {}", path);
                        }
                    });
                }
                Err(e) => {
                    println!("Connection error: {}", e);
                }
            }
        }
    }
}

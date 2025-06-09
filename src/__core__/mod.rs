pub mod handler;

use crate::__core__::handler::handle_connection;
use std::io::{Read};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

pub struct Route {
    path: String,
    handler: Box<dyn Fn(&mut TcpStream) + Send + Sync + 'static>,
}
pub struct Server {
    address: String,
    routes: Vec<Route>,
}

impl Server {
    // constructor
    pub fn new(address: String) -> Self {
        Self {
            address: address.to_string(),
            routes: Vec::new(),
        }
    }

    pub fn route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");

        println!("Server is running on {}", self.address);

        let routes = Arc::new(self.routes);

        for stream in listener.incoming() {
            let routes = Arc::clone(&routes);

            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        let mut buffer = [0; 1024];
                        let n = stream.read(&mut buffer).unwrap_or(0);
                        let req = String::from_utf8_lossy(&buffer[..n]);
                        let first = req.lines().next().unwrap_or("");
                        let path = first.split_whitespace().nth(1).unwrap_or("/");
                        let path = path.trim_start_matches('/');

                        if let Some(route) = routes.iter().find(|r| r.path == path) {
                            (route.handler)(&mut stream);
                        } else {
                            handle_connection(&mut stream);
                        }
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

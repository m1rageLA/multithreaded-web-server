pub mod handler;

use crate::server::handler::handle_connection;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

pub struct Server {
    address: String,
}

impl Server {
    // контруктор
    pub fn new(adress: String) -> Self {
        Self {
            address: adress.to_string(),
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).expect("Failed to bind address");

        println!("Server is running on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || {
                        handle_connection(&mut stream);
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}

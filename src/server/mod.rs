use std::net::TcpListener;
use std::thread;

mod handler;
pub fn run_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Failed to start TCP listener on port 7878");

    println!("Server running on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");
                thread::spawn(move || handler::handle_connection(&mut stream));
            }
            Err(e) => {
                println!("Error connecting to client: {}", e);
            }
        }
    }
}

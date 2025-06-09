// controllers/indexController.rs
use std::net::TcpStream;

pub fn index_controller() -> impl Fn(&mut TcpStream) + Send + Sync + 'static {
    |stream| {
        // обработка запроса
    }
}

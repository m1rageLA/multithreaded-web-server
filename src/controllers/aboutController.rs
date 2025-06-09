// controllers/indexController.rs
use std::net::TcpStream;

pub fn about_controller() -> impl Fn(&mut TcpStream) + Send + Sync + 'static {
    |stream| {
        // обработка запроса
    }
}

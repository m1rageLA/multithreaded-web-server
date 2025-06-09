use std::net::TcpStream;

pub struct Route {
    pub path: String,
    pub handler: Box<dyn Fn(&mut TcpStream) + Send + Sync + 'static>,
}

pub struct App {
    pub address: String,
    pub routes: Vec<Route>,
}
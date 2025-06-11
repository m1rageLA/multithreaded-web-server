use std::{net::TcpStream, sync::Arc};

pub struct Route {
    pub path: String,
    pub handler: Arc<
        dyn Fn(Request, &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
            + Send
            + Sync,
    >,
}
pub struct App {
    pub address: String,
    pub routes: Vec<Route>,
}
pub struct Request<'a> {
    pub stream: &'a mut TcpStream,
    pub path: String,
    pub method: String,
}
pub struct Response<'a> {
    pub stream: &'a mut TcpStream,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

use std::net::TcpStream;
use crate::__core__::{Route, App};

impl App {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream) + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }
}

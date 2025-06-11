use std::sync::Arc;

use crate::__core__::App;
use crate::__core__::Route;
use crate::__core__::types::{Request, Response};

impl App {
    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, &mut Response) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync + 'static,
    {
        self.routes.push(Route {
            path: path.to_string(),
            handler: Arc::new(handler),
        });
    }
}

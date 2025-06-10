mod __core__;
use __core__::App;

mod controllers;
fn main() {
    let mut app = App::new("127.0.0.1:7878".to_string());

    app.get("/", |req, res| {
        // res.send();
        Err("Error".into())
    });

    app.run();
}

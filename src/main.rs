mod __core__;
use __core__::{App};
fn main() {
    let mut app = App::new("127.0.0.1:7878".to_string());

    app.get("", |req, mut res| {
        res.send("Hello world from my first method!");
        Err("Error".into())
    }); 

    app.run();
}

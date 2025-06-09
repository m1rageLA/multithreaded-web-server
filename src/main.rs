mod __core__;
use __core__::Server;
use controllers::indexController::index_controller;
use controllers::aboutController::about_controller;

mod controllers; 
fn main() {
    let mut app = Server::new("127.0.0.1:7878".to_string());
    
    app.route("", index_controller());
    app.route("about", about_controller());

    app.run();
}

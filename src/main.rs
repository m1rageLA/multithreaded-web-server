mod __core__;
use __core__::App;

use controllers::indexController::index_controller;
use controllers::aboutController::about_controller;

mod controllers; 
fn main() {
    let mut app = App::new("127.0.0.1:7878".to_string());
    
    app.get("/", index_controller());
    app.get("/about", about_controller());

    app.run();
}

mod server;

fn main() {
    let address = "127.0.0.1:7878";
    server::run_server(address);
}
mod handler;
mod router;
mod server;
use std::env;

use dotenv::dotenv;
use server::Server;

fn main() {
    dotenv().ok();
    let addr = env::var("API_URL").expect("API_URL must be set");
    let server = Server::new(&addr);
    server.run();
}

use apirs::server::init_server;
use rocket::Error;
use std::process::exit;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // start the server
    match init_server().await {
        Ok(server) => server.launch().await,
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}

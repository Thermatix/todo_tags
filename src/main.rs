use std::env;

mod gprc;
mod server;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let app = app::new();
    let mut server = server::Server::new(server_addr());
    server.start();
    app.run(env::args().collect());
    Ok(())
}

fn server_addr() -> String {
    match env::var("TDT_SERVER_ADDRESS") {
        Ok(addr) => addr.into(),
        _ => "[::1]:50051".into()
    }
}



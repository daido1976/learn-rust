use std::net::TcpListener;
use zero2prod::{app, config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = config::get_configuration();
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    app::run(listener)?.await
}

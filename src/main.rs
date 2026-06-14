mod client;
mod command;
mod config;
mod errors;
mod fundamental_types;
mod server;

use tokio::{io, net::TcpListener};

use crate::config::ClientConfig;
use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")), // Fallback level
        )
        .with_line_number(true)
        .with_target(true)
        .with_file(true)
        .init();

    let host = "127.0.0.1";
    let port = "6379";

    let listener = match TcpListener::bind(format!("{}:{}", host, port)).await {
        Err(err) => {
            tracing::error!("failed to bind on addr [{}:{}]: {}", host, port, err);
            std::process::exit(1);
        }
        Ok(socket) => {
            tracing::debug!("listening socket binded on {}:{}", host, port);
            socket
        }
    };

    let config = ClientConfig {
        client_buff_initial_size: 50,
    };

    let server = Server::new(listener, config);
    tracing::info!("starting server listen {}:{}", host, port);
    server.listen_and_serve().await;
    Ok(())
}

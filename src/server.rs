use tokio::net::TcpListener;
use tokio_stream::{StreamExt, wrappers::TcpListenerStream};

use crate::client::Client;
use crate::config::ClientConfig;

#[derive(Debug)]
pub struct Server {
    listener: TcpListenerStream,
    client_configs: ClientConfig,
}

impl Server {
    pub fn new(listener: TcpListener, client_configs: ClientConfig) -> Self {
        Server {
            listener: TcpListenerStream::new(listener),
            client_configs,
        }
    }

    pub async fn listen_and_serve(mut self) {
        while let Some(conn_result) = self.listener.next().await {
            let new_conn = match conn_result {
                Ok(conn) => conn,
                Err(err) => {
                    tracing::error!("new connection failed: {}", err);
                    continue;
                }
            };

            tokio::task::spawn(async move {
                let client = Client::new(new_conn);
                tracing::info!("accepted new connection");
                client.handle_connection().await;
            });
        }
    }
}

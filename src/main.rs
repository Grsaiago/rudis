use tokio::{
    io::{self, AsyncWriteExt, BufStream},
    net::{TcpListener, TcpStream},
};
use tokio_stream::{StreamExt, wrappers::TcpListenerStream};
mod redis_types;

#[derive(Debug)]
struct Server {
    listener: TcpListenerStream,
}

impl Server {
    fn new(listener: TcpListener) -> Self {
        Server {
            listener: TcpListenerStream::new(listener),
        }
    }
}

struct Client {
    conn: BufStream<TcpStream>,
}

impl Client {
    fn new(conn: TcpStream) -> Self {
        Self {
            conn: BufStream::new(conn),
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_target(true)
        .with_file(true)
        .init();

    let host = "127.0.0.1";
    let port = "6379";

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .inspect_err(|err| {
            tracing::error!("failed to bind on addr [{}:{}]: {}", host, port, err)
        })?;
    tracing::info!("listening on {}:{}", host, port);

    let mut server = Server::new(listener);
    while let Some(conn_result) = server.listener.next().await {
        let new_conn = match conn_result {
            Ok(conn) => conn,
            Err(err) => {
                tracing::error!("new connection failed: {}", err);
                continue;
            }
        };
        tokio::task::spawn(async move {
            let mut client = Client::new(new_conn);
            tracing::info!("accepted and constructed new client");
            if let Err(err) = client.conn.write(b"bem vindo ao servidor ihuuuuu\n").await {
                tracing::error!("failed to write to client buffer: {}", err);
            }
            if let Err(err) = client.conn.flush().await {
                tracing::error!("failed to write to client socket: {}", err);
            }
        });
    }

    Ok(())
}

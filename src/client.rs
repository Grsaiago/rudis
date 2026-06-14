use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::commands::Command;
use crate::errors::ClientRequestErr;
use crate::fundamental_types::{ArrayDataType, RedisType};

pub struct Client {
    pub conn: TcpStream,
    pub buff: Vec<u8>,
}

impl Client {
    pub fn new(conn: TcpStream) -> Self {
        let buff = Vec::with_capacity(50);
        let _ = conn.set_nodelay(true).inspect_err(|err| {
            tracing::debug!("failed to set TCP_NODELAY: {}", err);
        });
        Self { conn, buff }
    }

    pub async fn handle_connection(mut self) {
        loop {
            let resp_request = match self.read_request().await {
                Ok(val) => val,
                Err(err) => {
                    tracing::debug!("failed to process client request {}", err.to_string());
                    return;
                }
            };
            tracing::debug!("received client message {:?}", resp_request);
            let request = match resp_request {
                RedisType::Array(value) => value,
                fallback => {
                    tracing::debug!(
                        "expected request as {}, got {}",
                        std::any::type_name::<ArrayDataType>(),
                        fallback.type_name()
                    );
                    return;
                }
            };
            let cmd = Command::try_from(request);
            tracing::debug!("received request {:?}", cmd);
        }
    }

    pub async fn read_request(&mut self) -> Result<RedisType, ClientRequestErr> {
        let mut tmp_buff = [0u8; 50];
        loop {
            let n = timeout(
                tokio::time::Duration::from_secs(20),
                self.conn.read(&mut tmp_buff),
            )
            .await??;

            self.buff.extend(&tmp_buff[..n]);
            match RedisType::parse(str::from_utf8(&self.buff).unwrap()) {
                Ok((_, parsed_value)) => return Ok(parsed_value),
                Err(err) => match err {
                    nom::Err::Incomplete(_) => {
                        tracing::debug!(
                            "incomplete parsing of client message [{:?}]",
                            str::from_utf8(&self.buff).unwrap()
                        );
                        continue;
                    }
                    nom::Err::Error(inner_parse_err) | nom::Err::Failure(inner_parse_err) => {
                        tracing::debug!("failed to parse client message [{:?}]", self.buff);
                        return Err(ClientRequestErr::ParseError(
                            inner_parse_err.input.len() as u32
                        ));
                    }
                },
            }
        }
    }
}

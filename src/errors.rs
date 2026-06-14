use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientRequestErr {
    #[error("client read timed out on {0}")]
    RecvTimeout(#[from] tokio::time::error::Elapsed),

    #[error("underlying socket failed with error [{0}]")]
    RecvError(#[from] std::io::Error),

    #[error("invalid resp2 character at {0} bytes")]
    ParseError(u32),
}

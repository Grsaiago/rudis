use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientRequestErr {
    #[error("client timed on {0}")]
    RecvTimeoutErr(#[from] tokio::time::error::Elapsed),

    #[error("client disconnected [{0}]")]
    DisconectErr(#[from] std::io::Error),

    #[error("invalid resp2 character at {0} bytes")]
    ParseError(u32),
}

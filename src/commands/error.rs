use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("invalid arguments: expected [{expected}], got [{got}].")]
    InvalidArgs { expected: String, got: String },

    #[error("execution failed with message: {0}.")]
    ExecutionFailed(String),
}

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CommandExecutionError {
    #[error("execution failed with message: {0}.")]
    ExecutionFailed(String),
}

#[derive(Error, Debug, Clone)]
pub enum CommandParseError {
    #[error("invalid arguments: {0} ")]
    InvalidArgs(#[from] InvalidArgsError),

    #[error("execution failed with message: {0}.")]
    ExecutionFailed(String),

    #[error("empty command")]
    EmptyCommand,

    #[error("{0} is not a supported command")]
    NonSupportedCommand(String),
}

#[derive(Error, Debug, Clone)]
pub enum InvalidArgsError {
    #[error("invalid number of arguments: expected {expected}, got {got}.")]
    NumberOfArgs { expected: String, got: usize },

    #[error("invalid type of arguments: expected {expected}, got {got}.")]
    TypeOfArgs { expected: String, got: String },
}

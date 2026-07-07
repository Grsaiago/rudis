use crate::{
    commands::{
        error::{CommandParseError, InvalidArgsError},
        ping::PingCommandDataType,
    },
    fundamental_types::{ArrayDataType, RedisType},
};

#[derive(Debug, Clone)]
pub enum Command {
    Ping(PingCommandDataType),
}

impl Command {
    const PING_IDENTIFIER: &str = "PING";
}

impl TryFrom<ArrayDataType> for Command {
    type Error = CommandParseError;

    fn try_from(value: ArrayDataType) -> Result<Self, Self::Error> {
        let mut it = value.into_iter();
        let cmd_name = match it.next() {
            None => return Err(CommandParseError::EmptyCommand),
            Some(RedisType::BulkString(cmd_name)) => cmd_name,
            Some(fallback) => {
                tracing::error!(
                    type_received = fallback.type_name(),
                    "this codepath was supposed to be unreachable ..."
                );
                return Err(CommandParseError::InvalidArgs(
                    InvalidArgsError::TypeOfArgs {
                        expected: "BulkString".to_string(),
                        got: fallback.type_name().to_string(),
                    },
                ));
            }
        };

        let cmd_args: ArrayDataType = it.collect();
        match cmd_name.as_ref() {
            Command::PING_IDENTIFIER => {
                tracing::debug!("received ping command with args {:?}", &cmd_args);
                return Ok(Command::Ping(cmd_args.try_into()?));
            }
            fallback => {
                tracing::debug!("received non-supported command [{}]", fallback);
                return Err(CommandParseError::NonSupportedCommand(fallback.to_string()));
            }
        }
    }
}

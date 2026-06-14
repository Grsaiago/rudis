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

impl TryFrom<ArrayDataType> for Command {
    type Error = CommandParseError;

    fn try_from(mut value: ArrayDataType) -> Result<Self, Self::Error> {
        if value.0.is_empty() {
            return Err(CommandParseError::EmptyCommand);
        }

        match &value.0[0] {
            RedisType::BulkString(cmd_name) => {
                let cmd_name = cmd_name.as_ref().to_lowercase();
                value.0.remove(0);
                let cmd_args = ArrayDataType::from(value.0);

                match cmd_name.as_ref() {
                    "ping" => {
                        tracing::debug!("received ping command with args {:?}", &cmd_args);
                        Ok(Command::Ping(cmd_args.try_into()?))
                    }
                    fallback => {
                        tracing::debug!("received non-supported command [{}]", fallback);
                        Err(CommandParseError::NonSupportedCommand(fallback.to_string()))
                    }
                }
            }
            fallback => {
                tracing::error!(
                    type_received = fallback.type_name().to_string(),
                    "this codepath was supposed to be unreachable since every client message is sent as a an array of bulk strings"
                );
                Err(CommandParseError::InvalidArgs(
                    InvalidArgsError::TypeOfArgs {
                        expected: "BulkString".to_string(),
                        got: fallback.type_name().to_string(),
                    },
                ))
            }
        }
    }
}

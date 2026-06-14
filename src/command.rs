use crate::fundamental_types::{BulkStringDataType, RedisType};

#[derive(Debug, Clone, PartialEq)]
pub enum CommandName {
    Ping,
    Get,
    Set,
}

impl TryFrom<BulkStringDataType> for CommandName {
    type Error = String;

    fn try_from(s: BulkStringDataType) -> Result<Self, Self::Error> {
        match s.as_ref() {
            "PING" => Ok(CommandName::Ping),
            "GET" => Ok(CommandName::Get),
            "SET" => Ok(CommandName::Set),
            _ => Err(format!("Unknown command: {}", s.as_ref())),
        }
    }
}

pub struct Command {
    pub name: CommandName,
    pub args: Vec<RedisType>,
}

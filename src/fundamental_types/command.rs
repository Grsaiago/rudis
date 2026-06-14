use crate::fundamental_types::RedisType;

#[derive(Debug, Clone, PartialEq)]
pub enum CommandName {
    Ping,
    Get,
    Set,
}

impl TryFrom<&str> for CommandName {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "PING" => Ok(CommandName::Ping),
            "GET" => Ok(CommandName::Get),
            "SET" => Ok(CommandName::Set),
            _ => Err(format!("Unknown command: {}", s)),
        }
    }
}

pub struct Command {
    pub name: CommandName,
    pub args: Vec<RedisType>,
}

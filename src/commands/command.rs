use crate::commands::ping::PingCommandDataType;

pub enum Command {
    Ping(PingCommandDataType),
}

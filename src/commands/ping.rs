use crate::commands::CommandError;
use crate::fundamental_types::{ArrayDataType, BulkStringDataType, RedisType};

#[derive(Debug, Clone)]
pub struct PingCommandDataType(Option<BulkStringDataType>);

impl TryFrom<ArrayDataType> for PingCommandDataType {
    type Error = CommandError;

    fn try_from(value: ArrayDataType) -> Result<Self, Self::Error> {
        match value.0.len() {
            0 => Ok(Self(None)),
            1 => match &value.0[1] {
                RedisType::BulkString(s) => Ok(Self(Some(s.clone()))),
                // RedisType::BulkString(s) => Ok(Self(Some(BulkStringDataType::new(s.as_ref())))),
                _ => Err(CommandError::InvalidArgs {
                    expected: "BulkString".to_string(),
                    got: "non-BulkString".to_string(),
                }),
            },
            _ => Err(CommandError::InvalidArgs {
                expected: "0 or 1 argument".to_string(),
                got: format!("{} arguments", value.0.len()),
            }),
        }
    }
}

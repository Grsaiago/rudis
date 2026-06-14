use crate::commands::error::InvalidArgsError;
use crate::fundamental_types::{ArrayDataType, BulkStringDataType, RedisType};

#[derive(Debug, Clone)]
pub struct PingCommandDataType(Option<BulkStringDataType>);

impl TryFrom<ArrayDataType> for PingCommandDataType {
    type Error = InvalidArgsError;

    fn try_from(value: ArrayDataType) -> Result<Self, Self::Error> {
        match value.0.len() {
            0 => Ok(Self(None)),
            1 => match &value.0[1] {
                RedisType::BulkString(s) => Ok(Self(Some(s.clone()))),
                _ => Err(InvalidArgsError::TypeOfArgs {
                    expected: std::any::type_name::<BulkStringDataType>().to_string(),
                    got: value.0[1].type_name().to_string(),
                }),
            },
            _ => Err(InvalidArgsError::NumberOfArgs {
                expected: "0 or 1 arguments".to_string(),
                got: value.0.len(),
            }),
        }
    }
}

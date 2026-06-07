mod bulk_string;
mod integer;
mod simple_error;
mod simple_string;

pub use bulk_string::BulkStringDataType;
pub use integer::IntegerDataType;
pub use simple_error::SimpleErrorDataType;
pub use simple_string::SimpleStringDataType;

use nom::{IResult, Parser, branch::alt};

#[derive(Debug, PartialEq)]
pub enum RedisType {
    SimpleString(SimpleStringDataType),
    SimpleError(SimpleErrorDataType),
    Integer(IntegerDataType),
}

impl RedisType {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        alt((
            SimpleStringDataType::parse,
            SimpleErrorDataType::parse,
            IntegerDataType::parse,
        ))
        .parse(input)
    }
}

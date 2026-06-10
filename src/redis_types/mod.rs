mod array;
mod bulk_string;
mod integer;
mod null_strings;
mod simple_error;
mod simple_string;

pub use array::ArrayDataType;
pub use bulk_string::BulkStringDataType;
pub use integer::IntegerDataType;
pub use null_strings::NullBulkStringData;
pub use simple_error::SimpleErrorDataType;
pub use simple_string::SimpleStringDataType;

use nom::{IResult, Parser, branch::alt};

#[derive(Debug, PartialEq)]
pub enum RedisType {
    Array(ArrayDataType),
    NullBulkString(NullBulkStringData),
    BulkString(BulkStringDataType),
    Integer(IntegerDataType),
    SimpleString(SimpleStringDataType),
    SimpleError(SimpleErrorDataType),
}

impl RedisType {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        alt((
            ArrayDataType::parse,
            NullBulkStringData::parse, // this has to be before the BulkString because They're the same identifier
            BulkStringDataType::parse,
            IntegerDataType::parse,
            SimpleErrorDataType::parse,
            SimpleStringDataType::parse,
        ))
        .parse(input)
    }
}

use nom::{IResult, bytes::streaming::tag};

use crate::fundamental_types::RedisType;

#[derive(Debug, PartialEq, Eq)]
pub struct NullBulkStringData;

impl NullBulkStringData {
    const RESP_IDENTIFIER: &str = "$-1\r\n";

    pub fn new() -> Self {
        Self
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;
        Ok((input, RedisType::NullBulkString(NullBulkStringData::new())))
    }
}

#[cfg(test)]
mod test {
    use crate::fundamental_types::{NullBulkStringData, RedisType};

    #[test]
    fn simple_parse() {
        let input = "$-1\r\n";

        assert_eq!(
            Ok(("", RedisType::NullBulkString(NullBulkStringData))),
            NullBulkStringData::parse(input)
        );
    }
}

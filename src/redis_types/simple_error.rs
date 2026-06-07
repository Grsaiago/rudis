use crate::redis_types::RedisType;
use nom::{
    IResult, Parser,
    bytes::{tag, take_until},
};

#[derive(Debug, PartialEq)]
pub struct SimpleErrorDataType(String);

impl SimpleErrorDataType {
    const RESP_IDENTIFIER: &str = "-";

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER).parse(input)?;

        let (input, string) = take_until("\r\n").parse(input)?;

        Ok((
            input,
            RedisType::SimpleError(SimpleErrorDataType::new(string.to_string())),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::redis_types::{RedisType, SimpleErrorDataType};

    #[test]
    fn simple_parse() {
        let input = "-asdasdasdasd\r\n";

        let result = SimpleErrorDataType::parse(input);
        assert!(result.is_ok());
        let (_, parsed) = result.unwrap();

        let expected = RedisType::SimpleError(SimpleErrorDataType::new(
            input[1..input.len() - 2].to_string(),
        ));
        assert_eq!(expected, parsed)
    }
}

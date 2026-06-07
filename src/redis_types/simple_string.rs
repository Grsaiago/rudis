use crate::redis_types::RedisType;
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};

#[derive(Debug, PartialEq)]
pub struct SimpleStringDataType(String);

impl SimpleStringDataType {
    const RESP_IDENTIFIER: &str = "+";

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;

        let (input, string) = take_until("\r\n")(input)?;

        Ok((
            input,
            RedisType::SimpleString(SimpleStringDataType::new(string.to_string())),
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::redis_types::{RedisType, SimpleStringDataType};

    #[test]
    fn simple_parse() {
        let input = "+asdasdasdasd\r\n";

        let result = SimpleStringDataType::parse(input);
        assert!(result.is_ok());
        let (_, parsed) = result.unwrap();

        let expected = RedisType::SimpleString(SimpleStringDataType::new(
            input[1..input.len() - 2].to_string(),
        ));
        assert_eq!(expected, parsed)
    }
}

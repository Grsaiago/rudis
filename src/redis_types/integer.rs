use nom::{
    IResult, Parser,
    bytes::{tag, take_until},
    error::{Error, ErrorKind},
};

use crate::redis_types::RedisType;

#[derive(Debug, PartialEq)]
pub struct IntegerDataType(i64);

impl IntegerDataType {
    const RESP_IDENTIFIER: &str = ":";

    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER).parse(input)?;

        let (input, integer_str) = take_until("\r\n").parse(input)?;

        match integer_str.parse::<i64>() {
            Ok(parsed_int) => Ok((input, RedisType::Integer(IntegerDataType::new(parsed_int)))),
            Err(_) => Err(nom::Err::Failure(Error::new(input, ErrorKind::MapRes))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::redis_types::{IntegerDataType, RedisType};

    #[test]
    pub fn i64_max() {
        let integer_value = i64::MAX;
        let input = format!(":{}\r\n", integer_value.to_string());

        let result = IntegerDataType::parse(&input);
        assert!(result.is_ok());
        let (_, parsed) = result.unwrap();

        let expected = RedisType::Integer(IntegerDataType::new(integer_value));
        assert_eq!(expected, parsed)
    }

    #[test]
    pub fn i64_min() {
        let integer_value = i64::MIN;
        let input = format!(":{}\r\n", integer_value.to_string());

        let result = IntegerDataType::parse(&input);
        assert!(result.is_ok());
        let (_, parsed) = result.unwrap();

        let expected = RedisType::Integer(IntegerDataType::new(integer_value));
        assert_eq!(expected, parsed)
    }

    #[test]
    pub fn out_of_i64_range() {
        let integer_value = i128::MAX;
        let input = format!(":{}\r\n", integer_value.to_string());

        let result = IntegerDataType::parse(&input);
        assert!(result.is_err());
    }
}

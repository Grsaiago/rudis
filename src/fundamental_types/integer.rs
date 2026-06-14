use nom::{
    IResult,
    bytes::streaming::{tag, take_while},
    error::{Error, ErrorKind},
};

use crate::fundamental_types::RedisType;

#[derive(Debug, PartialEq, Eq)]
pub struct IntegerDataType(i64);

impl IntegerDataType {
    const RESP_IDENTIFIER: &str = ":";

    pub fn new<T: Into<i64>>(value: T) -> Self {
        Self(value.into())
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;
        let (input, integer_str) = take_while(|c: char| !c.is_ascii_whitespace())(input)?;
        let (input, _) = tag("\r\n")(input)?;

        match integer_str.parse::<i64>() {
            Ok(parsed_int) => Ok((input, RedisType::Integer(IntegerDataType::new(parsed_int)))),
            Err(_) => Err(nom::Err::Failure(Error::new(input, ErrorKind::MapRes))),
        }
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::fundamental_types::{IntegerDataType, RedisType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [
            TestCase {
                input: &i64::MIN.to_string(),
                expected: Ok(("", RedisType::Integer(IntegerDataType::new(i64::MIN)))),
            },
            TestCase {
                input: &i64::MAX.to_string(),
                expected: Ok(("", RedisType::Integer(IntegerDataType::new(i64::MAX)))),
            },
            TestCase {
                input: &(23 as i64).to_string(),
                expected: Ok(("", RedisType::Integer(IntegerDataType::new(23)))),
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let input = format!(":{}\r\n", case.input);

            let result = IntegerDataType::parse(&input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, &input
            );
        }
    }

    #[test]
    fn parse_fails_with_error_when_input_does_not_start_with_colon() {
        let input = "+42\r\n";
        let expected = Err(nom::Err::Error(nom::error::Error::new(
            "+42\r\n",
            nom::error::ErrorKind::Tag,
        )));

        let result = IntegerDataType::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_returns_incomplete_when_value_is_truncated() {
        let input = ":42";
        let expected = Err(nom::Err::Incomplete(nom::Needed::Size(
            std::num::NonZeroUsize::new(1).unwrap(),
        )));

        let result = IntegerDataType::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_fails_with_failure_when_value_is_not_an_integer() {
        let input = ":abc\r\n";
        let expected = Err(nom::Err::Failure(nom::error::Error::new(
            "",
            nom::error::ErrorKind::MapRes,
        )));

        let result = IntegerDataType::parse(input);
        assert_eq!(result, expected);
    }
}

use nom::{
    IResult,
    bytes::streaming::{tag, take, take_while1},
    error::{Error, ErrorKind},
};

use crate::redis_types::RedisType;

#[derive(Debug, PartialEq, Eq)]
pub struct BulkStringDataType(String);

impl BulkStringDataType {
    const RESP_IDENTIFIER: &str = "$";

    pub fn new<T: ToString>(value: T) -> Self {
        Self(value.to_string())
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;

        let (input, string_len_raw) = take_while1(|c: char| c.is_ascii_digit())(input)?;
        let string_len = match string_len_raw.parse::<u64>() {
            Ok(val) => val,
            Err(_) => return Err(nom::Err::Failure(Error::new(input, ErrorKind::MapRes))),
        };
        let (input, _) = tag("\r\n")(input)?;

        let (input, raw_string) = take(string_len)(input)?;
        let (input, _) = tag("\r\n")(input)?;

        return Ok((
            input,
            RedisType::BulkString(BulkStringDataType::new(raw_string.to_string())),
        ));
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::redis_types::{BulkStringDataType, RedisType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [
            TestCase {
                input: "oieoie",
                expected: Ok(("", RedisType::BulkString(BulkStringDataType::new("oieoie")))),
            },
            TestCase {
                input: "OIIIIIIIIIIII\n\r",
                expected: Ok((
                    "",
                    RedisType::BulkString(BulkStringDataType::new("OIIIIIIIIIIII\n\r")),
                )),
            },
            TestCase {
                input: "\r\noieoie\r\n",
                expected: Ok((
                    "",
                    RedisType::BulkString(BulkStringDataType::new("\r\noieoie\r\n")),
                )),
            },
            TestCase {
                input: "",
                expected: Ok(("", RedisType::BulkString(BulkStringDataType::new("")))),
            },
            TestCase {
                input: "'oieoieoie'",
                expected: Ok((
                    "",
                    RedisType::BulkString(BulkStringDataType::new("'oieoieoie'")),
                )),
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let input = format!(
                "${}\r\n{}\r\n",
                case.input.chars().count().to_string(),
                case.input
            );
            let result = BulkStringDataType::parse(&input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, &input
            );
        }
    }
}

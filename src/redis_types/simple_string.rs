use crate::redis_types::RedisType;
use nom::{
    IResult,
    bytes::streaming::{tag, take_until},
};

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleStringDataType(String);

impl SimpleStringDataType {
    const RESP_IDENTIFIER: &str = "+";

    pub fn new<T: ToString>(value: T) -> Self {
        Self(value.to_string())
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;
        let (input, string) = take_until("\r\n")(input)?;
        let (input, _) = tag("\r\n")(input)?;

        Ok((
            input,
            RedisType::SimpleString(SimpleStringDataType::new(string.to_string())),
        ))
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::redis_types::{RedisType, SimpleStringDataType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [
            TestCase {
                input: "aaaaa",
                expected: Ok((
                    "",
                    RedisType::SimpleString(SimpleStringDataType::new("aaaaa")),
                )),
            },
            TestCase {
                input: "oieoie",
                expected: Ok((
                    "",
                    RedisType::SimpleString(SimpleStringDataType::new("oieoie")),
                )),
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let input = format!("+{}\r\n", case.input);
            let result = SimpleStringDataType::parse(&input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, &input
            );
        }
    }
}

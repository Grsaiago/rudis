use crate::redis_types::RedisType;
use nom::{
    IResult,
    bytes::complete::{tag, take_until},
};

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleErrorDataType(String);

impl SimpleErrorDataType {
    const RESP_IDENTIFIER: &str = "-";

    pub fn new<T: ToString>(value: T) -> Self {
        Self(value.to_string())
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;
        let (input, string) = take_until("\r\n")(input)?;
        let (input, _) = tag("\r\n")(input)?;

        Ok((
            input,
            RedisType::SimpleError(SimpleErrorDataType::new(string.to_string())),
        ))
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::redis_types::{RedisType, SimpleErrorDataType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [
            TestCase {
                input: "asdasdasdasd",
                expected: Ok((
                    "",
                    RedisType::SimpleError(SimpleErrorDataType::new("asdasdasdasd")),
                )),
            },
            TestCase {
                input: "vishr",
                expected: Ok((
                    "",
                    RedisType::SimpleError(SimpleErrorDataType::new("vishr")),
                )),
            },
            TestCase {
                input: "deu ' ruim aqui",
                expected: Ok((
                    "",
                    RedisType::SimpleError(SimpleErrorDataType::new("deu ' ruim aqui")),
                )),
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let input = format!("-{}\r\n", case.input);

            let result = SimpleErrorDataType::parse(&input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, &input
            );
        }
    }
}

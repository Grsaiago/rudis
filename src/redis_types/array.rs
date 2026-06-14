use nom::{
    IResult,
    bytes::streaming::{tag, take_while},
    error::{Error, ErrorKind},
};

use crate::redis_types::RedisType;

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayDataType(Vec<RedisType>);

impl ArrayDataType {
    const RESP_IDENTIFIER: &str = "*";

    pub fn new<T: Iterator<Item = RedisType>>(value: T) -> Self {
        ArrayDataType(value.collect())
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER)(input)?;
        let (input, vector_len_raw) = take_while(|c: char| !c.is_ascii_whitespace())(input)?;
        let (input, _) = tag("\r\n")(input)?;
        let vector_len = match vector_len_raw.parse::<u64>() {
            Ok(val) => val,
            Err(_) => return Err(nom::Err::Error(Error::new(input, ErrorKind::MapRes))),
        };

        let (input, resp_vec) = (0..vector_len).try_fold(
            (input, Vec::with_capacity(vector_len as usize)),
            |(input, mut acc), _| {
                let (input, resp_type) = RedisType::parse(input)?;
                acc.push(resp_type);
                Ok((input, acc))
            },
        )?;

        Ok((
            input,
            RedisType::Array(ArrayDataType::new(resp_vec.into_iter())),
        ))
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::redis_types::{ArrayDataType, IntegerDataType, RedisType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            count: usize,
            elements: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [
            TestCase {
                count: 0,
                elements: "",
                expected: Ok((
                    "",
                    RedisType::Array(ArrayDataType::new(Vec::new().into_iter())),
                )),
            },
            TestCase {
                count: 1,
                elements: ":42\r\n",
                expected: Ok((
                    "",
                    RedisType::Array(ArrayDataType::new(
                        vec![RedisType::Integer(IntegerDataType::new(42))].into_iter(),
                    )),
                )),
            },
            TestCase {
                count: 2,
                elements: ":1\r\n:2\r\n",
                expected: Ok((
                    "",
                    RedisType::Array(ArrayDataType::new(
                        vec![
                            RedisType::Integer(IntegerDataType::new(1)),
                            RedisType::Integer(IntegerDataType::new(2)),
                        ]
                        .into_iter(),
                    )),
                )),
            },
            TestCase {
                count: 2,
                elements: "*1\r\n:1\r\n:2\r\n",
                expected: Ok((
                    "",
                    RedisType::Array(ArrayDataType::new(
                        vec![
                            RedisType::Array(ArrayDataType::new(
                                vec![RedisType::Integer(IntegerDataType::new(1))].into_iter(),
                            )),
                            RedisType::Integer(IntegerDataType::new(2)),
                        ]
                        .into_iter(),
                    )),
                )),
            },
        ];

        for (i, case) in cases.iter().enumerate() {
            let input = format!("*{}\r\n{}", case.count, case.elements);
            let result = ArrayDataType::parse(&input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, &input
            );
        }
    }

    #[test]
    fn parse_fails_with_error_when_element_starts_without_resp_identifier() {
        let input = "*1\r\n\n\r";
        let expected = Err(nom::Err::Error(nom::error::Error::new(
            "\n\r",
            nom::error::ErrorKind::Tag,
        )));

        let result = ArrayDataType::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_returns_incomplete_when_element_is_truncated() {
        let input = "*1\r\n:1";
        let expected = Err(nom::Err::Incomplete(nom::Needed::Size(
            std::num::NonZeroUsize::new(1).unwrap(),
        )));

        let result = ArrayDataType::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_fails_with_failure_when_element_contains_invalid_data() {
        let input = "*1\r\n:abc\r\n";
        let expected = Err(nom::Err::Failure(nom::error::Error::new(
            "",
            nom::error::ErrorKind::MapRes,
        )));

        let result = ArrayDataType::parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_returns_incomplete_when_element_data_is_absent() {
        let input = "*1\r\n";
        let expected = Err(nom::Err::Incomplete(nom::Needed::Size(
            std::num::NonZeroUsize::new(1).unwrap(),
        )));

        let result = ArrayDataType::parse(input);
        assert_eq!(result, expected);
    }
}

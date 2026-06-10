use nom::{
    IResult,
    bytes::complete::{tag, take_until},
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
        let (input, vector_len_raw) = take_until("\r\n")(input)?;
        let (input, _) = tag("\r\n")(input)?;
        let vector_len = match vector_len_raw.parse::<u64>() {
            Ok(val) => val,
            Err(_) => return Err(nom::Err::Failure(Error::new(input, ErrorKind::MapRes))),
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
}

use nom::{IResult, Parser, branch::alt};

use crate::fundamental_types::{
    ArrayDataType, BulkStringDataType, IntegerDataType, NullBulkStringData, SimpleErrorDataType,
    SimpleStringDataType,
};

#[derive(Debug, PartialEq, Eq)]
pub enum RedisType {
    Array(ArrayDataType),
    NullBulkString(NullBulkStringData),
    BulkString(BulkStringDataType),
    Integer(IntegerDataType),
    SimpleString(SimpleStringDataType),
    SimpleError(SimpleErrorDataType),
}

impl RedisType {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        alt((
            ArrayDataType::parse,
            NullBulkStringData::parse, // this has to be before the BulkString because They're the same identifier
            BulkStringDataType::parse,
            IntegerDataType::parse,
            SimpleErrorDataType::parse,
            SimpleStringDataType::parse,
        ))
        .parse(input)
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::fundamental_types::{NullBulkStringData, RedisType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [TestCase {
            input: "$-1\r\n",
            expected: Ok(("", RedisType::NullBulkString(NullBulkStringData::new()))),
        }];

        for (i, case) in cases.iter().enumerate() {
            let result = RedisType::parse(case.input);
            assert_eq!(
                result, case.expected,
                "case number {}, the input was `{}`",
                i, case.input
            );
        }
    }
}

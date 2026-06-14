use nom::{IResult, Parser, branch::alt};

use crate::fundamental_types::{
    ArrayDataType, BulkStringDataType, IntegerDataType, NullBulkStringDataType,
    SimpleErrorDataType, SimpleStringDataType,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RedisType {
    Array(ArrayDataType),
    NullBulkString(NullBulkStringDataType),
    BulkString(BulkStringDataType),
    Integer(IntegerDataType),
    SimpleString(SimpleStringDataType),
    SimpleError(SimpleErrorDataType),
}

impl RedisType {
    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        alt((
            ArrayDataType::parse,
            NullBulkStringDataType::parse, // this has to be before the BulkString because They're the same identifier
            BulkStringDataType::parse,
            IntegerDataType::parse,
            SimpleErrorDataType::parse,
            SimpleStringDataType::parse,
        ))
        .parse(input)
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            RedisType::Array(_) => "Array",
            RedisType::NullBulkString(_) => "Null Bulk String",
            RedisType::BulkString(_) => "Bulk String",
            RedisType::Integer(_) => "Integer",
            RedisType::SimpleString(_) => "Simple String",
            RedisType::SimpleError(_) => "Simple Error",
        }
    }
}

#[cfg(test)]
mod test {
    use nom::IResult;

    use crate::fundamental_types::{NullBulkStringDataType, RedisType};

    #[test]
    fn simple_parse() {
        struct TestCase<'a> {
            input: &'a str,
            expected: IResult<&'a str, RedisType>,
        }

        let cases = [TestCase {
            input: "$-1\r\n",
            expected: Ok(("", RedisType::NullBulkString(NullBulkStringDataType::new()))),
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

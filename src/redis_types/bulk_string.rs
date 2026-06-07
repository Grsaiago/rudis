use nom::{
    IResult, Parser,
    bytes::{
        complete::{take, take_while1},
        tag,
    },
    error::{Error, ErrorKind},
};

use crate::redis_types::RedisType;

pub struct BulkStringDataType(String);

impl BulkStringDataType {
    const RESP_IDENTIFIER: &str = "$";

    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn parse<'a>(input: &'a str) -> IResult<&'a str, RedisType> {
        let (input, _) = tag(Self::RESP_IDENTIFIER).parse(input)?;

        let (input, string_len_raw) = take_while1(|c: char| c.is_ascii_digit())(input)?;

        let string_len = match string_len_raw.parse::<u64>() {
            Ok(val) => val,
            Err(_) => return Err(nom::Err::Failure(Error::new(input, ErrorKind::MapRes))),
        };

        let (input, raw_string) = take(string_len)(input)?;
    }
}

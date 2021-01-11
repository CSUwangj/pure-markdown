#![allow(dead_code)]

use nom::error::ParseError;
use nom::IResult;
use nom::{InputLength, InputTake, Parser};

/// A parser similar to `nom::bytes::complete::take_until()`, but accept a 
/// parser as parameter, returns the input slice up to the postion where parser success.
/// It doesn't consume the pattern. It will return whole input if the pattern wasn't met.
/// # Example
/// ```rust
/// #[macro_use] extern crate nom;
/// use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use nom::bytes::complete::tag;
///
/// fn until_eof(s: &str) -> IResult<&str, &str> {
///     take_except(tag("eof"))(s)
/// }
/// assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
/// assert_eq!(until_eof("hello, world"), Ok(("", "hello, world")));
/// assert_eq!(until_eof(""), Ok(("", "")));
/// assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
/// ```
pub fn take_except<Input, Output, Error: ParseError<Input>, F: Parser<Input, Output, Error>>(
    parser: F,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputLength + InputTake,
    F: Fn(Input) -> IResult<Input, Output, Error>,
{
    move |i: Input| {
        let input = i;
        for index in 0..input.input_len() {
            let (rest, _front) = input.take_split(index);
            match parser(rest) {
                Ok(_) => return Ok(input.take_split(index)),
                Err(_) => continue,
            }
        }
        Ok(input.take_split(input.input_len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_except_with_char() {
        named!(exception<&str, char>, char!('c'));

        let input = "1234c";
        let expect_rest = "c";
        let expect_consumed = "1234";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);


        let input = "12345";
        let expect_rest = "";
        let expect_consumed = "12345";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);

        
    }

    #[test]
    fn test_except_with_tag() {
        named!(exception<&str, &str>, tag!("test"));

        let input = "1234test123";
        let expect_rest = "test123";
        let expect_consumed = "1234";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);

        let input = "1234123";
        let expect_rest = "";
        let expect_consumed = "1234123";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);
    }

    #[test]
    fn test_except_with_tuple() {
        named!(exception<&str, (&str, Vec<char>, &str)>, tuple!(
            tag!("test"),
            count!(char!('`'), 3),
            tag!("test")
        ));

        let input = "1234test123test```test123";
        let expect_rest = "test```test123";
        let expect_consumed = "1234test123";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);

        let input = "1234test123test``test123";
        let expect_rest = "";
        let expect_consumed = "1234test123test``test123";
        let (result_rest, result_consumed) = take_except(exception)(input).unwrap();
        assert_eq!(expect_rest, result_rest);
        assert_eq!(expect_consumed, result_consumed);
    }
}



#![allow(dead_code)]
use nom::{IResult, bytes::complete::take_while};
use crate::types::{Inline, Inline::Text};

pub fn text(input: &str) -> IResult<&str, Inline> {
  let (input, content) = take_while(|_ch| true)(input)?;
  Ok((input, Text(content.to_string())))
}

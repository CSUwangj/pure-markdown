#![allow(dead_code)]
use nom::{IResult, bytes::complete::take_while};
use crate::types::{Inline, Inline::{Text, Emphasis}};

pub fn emphasis_or_strong(input: &str) -> IResult<&str, Inline> {
  todo!()
}
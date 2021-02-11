
#![allow(dead_code)]
use nom::{IResult, bytes::complete::take_while};
use crate::types::{Inline, Inline::{Text}};

pub fn text(input: &str) -> IResult<&str, Inline> {
  let (input, content) = take_while(|_ch| true)(input)?;
  Ok((input, Text(content.to_string())))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_commonmark_ex647() {
    let input = "hello $.;'there";
    let want = Text("hello $.;'there".to_string());
    let (input, result) = text(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex648() {
    let input = "Foo χρῆν";
    let want = Text("Foo χρῆν".to_string());
    let (input, result) = text(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex649() {
    let input = "Multiple     spaces";
    let want = Text("Multiple     spaces".to_string());
    let (input, result) = text(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }
}
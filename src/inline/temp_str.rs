use nom::{IResult, bytes::complete::take_while};
use crate::types::{Inline, Inline::{Text}};

pub fn temp_str(input: &str) -> IResult<&str, Inline> {
  let (input, content) = take_while(|_ch| true)(input)?;
  Ok((input, Text(content.to_string())))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_take_all() {
    let input = "oiuvbdsoifb\n\tafd";
    let want = Text("oiuvbdsoifb\n\tafd".to_string());
    let (input, result) = temp_str(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }
}
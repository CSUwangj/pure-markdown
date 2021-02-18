use nom::{IResult, branch::alt};
use crate::types::{Inline, Inline::{Text, Emphasis}};

mod text;
mod emphasis_or_strong;

pub fn inline(input: &str) -> IResult<&str, Inline> {
  alt((text::text, emphasis_or_strong::emphasis_or_strong))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_commonmark_ex350() {
    let input = "*foo bar*";
    let want = Emphasis(vec![Text("foo bar".to_string())]);
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex351() {
    let input = "a * foo bar*";
    let want = Text("a * foo bar*".to_string());
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex356() {
    let input = "_foo bar_";
    let want = Emphasis(vec![Text("foo bar".to_string())]);
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex364() {
    let input = "_foo bar_";
    let want = Emphasis(vec![Text("foo bar".to_string())]);
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }
  
  #[test]
  fn test_commonmark_ex647() {
    let input = "hello $.;'there";
    let want = Text("hello $.;'there".to_string());
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex648() {
    let input = "Foo χρῆν";
    let want = Text("Foo χρῆν".to_string());
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }

  #[test]
  fn test_commonmark_ex649() {
    let input = "Multiple     spaces";
    let want = Text("Multiple     spaces".to_string());
    let (input, result) = inline(input).unwrap();
    assert_eq!(input, "");
    assert_eq!(want, result);
  }
}
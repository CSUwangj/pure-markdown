#![allow(dead_code)]
use crate::types::{Block, Block::CodeBlock};

use nom::{IResult, bytes::complete::{tag, take_till, take_while, take_while_m_n}, character::complete::char, combinator::opt, multi::{fold_many1, many0_count, many1}, sequence::tuple};

pub fn indented_chunk(input: &str) -> IResult<&str, String> {
    let (input, lines) = many1(tuple((
        take_while_m_n(4, 4, |ch| ch == ' '),
        take_till(|ch| ch == '\n'),
        opt(tag("\n")),
    )))(input)?;
    let content: Vec<String> = lines.iter().map(|line| line.1.to_string() + line.2.unwrap_or("")).collect();
    let content = content.concat();
    Ok((input, content.to_string()))
}

pub fn blanklines(input: &str) -> IResult<&str, usize> {
    let (input, count) = many0_count(tuple((take_while(|ch| ch == ' '), char('\n'))))(input)?;
    Ok((input, count))
}

pub fn indented_code_block(input: &str) -> IResult<&str, Block> {
    let (input, chunks) = fold_many1(
        tuple((indented_chunk, blanklines)),
        Vec::new(),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)?;

    let mut lines = vec![];
    chunks.iter().for_each(|chunk| {
        let (chunk, blank) = chunk;
        lines.push(chunk.to_string());
        for _i in 0..*blank {
            lines.push("\n".to_string());
        }
    });
    let content = lines.concat();
    Ok((input, CodeBlock(None, content)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_commonmark_ex77() {
        let input = "    a simple
      indented code block";
        let want = CodeBlock(
            None,
            "a simple
  indented code block"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex80() {
        let input = "    <a/>
    *hi*

    - one";
        let want = CodeBlock(
            None,
            "<a/>
*hi*

- one"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex81() {
        let input = "    chunk1

    chunk2
  
 
 
    chunk3
";
        let want = CodeBlock(
            None,
            "chunk1

chunk2



chunk3
"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex82() {
        let input = "    chunk1
      
      chunk2";
        let want = CodeBlock(
            None,
            "chunk1
  
  chunk2"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex84() {
        let input = "    foo
bar";
        let want = CodeBlock(
            None,
            "foo\n"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "bar");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex86() {
        let input = "        foo
    bar";
        let want = CodeBlock(
            None,
            "    foo
bar"
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex88() {
        let input = "    foo  ";
        let want = CodeBlock(
            None,
            "foo  "
                .to_string(),
        );
        let (input, result) = indented_code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
}

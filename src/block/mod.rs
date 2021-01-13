use crate::parser_combinator::take_except;
use crate::types::{Block, Block::CodeBlock};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::char,
    combinator::{eof},
    multi::{count, many0_count},
    sequence::tuple,
    IResult,
};

fn block_end(
    input: &str,
    c: char,
    cnt: usize,
) -> IResult<&str, (char, Vec<char>, &str, &str, &str)> {
    tuple((
        char('\n'),
        count(char(c), 3 + cnt),
        take_while(|ch| ch == c),
        take_while(|ch| ch == ' '),
        alt((eof, tag("\n"))),
    ))(input)
}

fn block_end_parser(
    c: char,
    count: usize,
) -> impl Fn(&str) -> IResult<&str, (char, Vec<char>, &str, &str, &str)> {
    move |s: &str| block_end(s, c, count)
}

pub fn code_block(input: &str) -> IResult<&str, Block> {
    named!(peek_char<&str, char>, peek!(one_of!("~`")));
    named!(code_lang<&str, &str>, take_while!(|c| c != '\n'));
    let (_, c) = peek_char(input).unwrap();
    let (input, (_, count)) = tuple((count(char(c), 3), many0_count(char(c))))(input)?;
    let (input, name) = code_lang(input)?;
    let name = name.trim();
    let (input, _) = char('\n')(input)?;

    let (input, content) = take_except(block_end_parser(c, count))(input)?;
    let (input, _) = opt!(input, block_end_parser(c, count))?;
    Ok((
        input,
        CodeBlock(Some(name.to_string()), content.to_string()),
    ))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_commonmark_ex88_5() {
        let input = "```
<
 >
```
";
        let want = CodeBlock(
            Some("".to_string()),
            "<
 >"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex89() {
        let input = "```
<
 >
```";
        let want = CodeBlock(
            Some("".to_string()),
            "<
 >"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex90() {
        let input = "~~~
<
 >
~~~";
        let want = CodeBlock(
            Some("".to_string()),
            "<
 >"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex92() {
        let input = "```
aaa
~~~
```";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
~~~"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex93() {
        let input = "~~~
aaa
```
~~~";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
```"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex93_5() {
        let input = "````
aaa
```
``````
";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
```"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex94() {
        let input = "````
aaa
```
``````";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
```"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex95() {
        let input = "~~~~
aaa
~~~
~~~~";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
~~~"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
}

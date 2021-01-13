#![allow(dead_code)]
use crate::parser_combinator::take_except;
use crate::types::{Block, Block::CodeBlock};
use nom::{IResult, branch::alt, bytes::complete::{tag, take_while}, character::complete::{char, one_of}, combinator::{eof, opt, peek}, multi::{count, many0_count}, sequence::tuple};

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

fn peek_char(input: &str) -> IResult<&str, char> {
  peek(one_of("~`"))(input)
}

fn info_string(input: &str) -> IResult<&str, &str> {
  take_while(|c| c != '\n')(input)
}

pub fn code_block(input: &str) -> IResult<&str, Block> {
    let (_, c) = peek_char(input).unwrap();
    let (input, (_, count)) = tuple((count(char(c), 3), many0_count(char(c))))(input)?;
    let (input, info) = info_string(input)?;
    let info = info.trim_matches(|ch| ch == c || ch == ' ');

    // empty code block
    match peek(block_end_parser(c, count))(input) {
        Ok(_) => {
            let (input, _) = block_end_parser(c, count)(input)?;
            return Ok((input, CodeBlock(Some(info.to_string()), "".to_string())));
        }
        Err(_) => {}
    };

    let (input, _) = char('\n')(input)?;
    let (input, content) = take_except(block_end_parser(c, count))(input)?;
    let (input, _) = opt(block_end_parser(c, count))(input)?;
    Ok((
        input,
        CodeBlock(Some(info.to_string()), content.to_string()),
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

    #[test]
    fn test_commonmark_ex96() {
        let input = "~~~~
aaa
~~~
";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
~~~
"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex97() {
        let input = "`````

```
aaa";
        let want = CodeBlock(
            Some("".to_string()),
            "
```
aaa"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex99() {
        let input = "```

  
```";
        let want = CodeBlock(
            Some("".to_string()),
            "
  "
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex100() {
        let input = "```
```";
        let want = CodeBlock(Some("".to_string()), "".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex112() {
        let input = "```ruby
def foo(x)
  return 3
end
```";
        let want = CodeBlock(
            Some("ruby".to_string()),
            "def foo(x)
  return 3
end"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex113() {
        let input = "~~~~    ruby startline=3 $%@#$
def foo(x)
  return 3
end
~~~~~~~";
        let want = CodeBlock(
            Some("ruby startline=3 $%@#$".to_string()),
            "def foo(x)
  return 3
end"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex114() {
        let input = "````;
````";
        let want = CodeBlock(Some(";".to_string()), "".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex115() {
        let input = "``` aa ```
foo";
        let want = CodeBlock(Some("aa".to_string()), "foo".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex116() {
        let input = "~~~ aa ~~~
foo";
        let want = CodeBlock(Some("aa".to_string()), "foo".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex117() {
        let input = "```
``` aaa
```";
        let want = CodeBlock(Some("".to_string()), "``` aaa".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
}

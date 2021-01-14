#![allow(dead_code)]
use crate::parser_combinator::take_except;
use crate::types::{Block, Block::CodeBlock};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{char, one_of},
    combinator::{eof, opt, peek},
    multi::{count, many0_count},
    sequence::tuple,
    IResult,
};

fn block_end(
    input: &str,
    c: char,
    cnt: usize,
) -> IResult<&str, (char, &str, Vec<char>, &str, &str, &str)> {
    tuple((
        char('\n'),
        take_while_m_n(0, 3, |ch| ch == ' '),
        count(char(c), 3 + cnt),
        take_while(|ch| ch == c),
        take_while(|ch| ch == ' '),
        alt((eof, tag("\n"))),
    ))(input)
}

fn block_end_parser(
    c: char,
    count: usize,
) -> impl Fn(&str) -> IResult<&str, (char, &str, Vec<char>, &str, &str, &str)> {
    move |s: &str| block_end(s, c, count)
}

fn peek_char(input: &str) -> IResult<&str, char> {
    let (input, (_, c)) = peek(tuple((take_while_m_n(0, 3, |ch| ch == ' '), one_of("~`"))))(input)?;
    Ok((input, c))
}

fn info_string(input: &str) -> IResult<&str, &str> {
    take_while(|c| c != '\n')(input)
}

fn remove_spaces(count: usize, content: &str) -> String {
    let mut content = content;
    let mut content_string: String = content.to_string();
    for _ in 0..count {
        content_string = content.replace("\n ", "\n");
        content = content_string.as_str();
    }
    content_string
}

pub fn code_block(input: &str) -> IResult<&str, Block> {
    let (_, c) = peek_char(input)?;
    let (input, (spaces, _, count)) = tuple((
        take_while_m_n(0, 3, |c| c == ' '),
        count(char(c), 3),
        many0_count(char(c)),
    ))(input)?;
    let (input, info) = info_string(input)?;
    let info = info.trim_matches(|ch| ch == c || ch == ' ');

    let (input, content) = take_except(block_end_parser(c, count))(input)?;
    let (input, _) = opt(block_end_parser(c, count))(input)?;

    let content = remove_spaces(spaces.len(), content);
    let content = match content.chars().nth(0) {
        Some('\n') => &content[1..],
        _ => content.as_str(),
    };

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
    fn test_commonmark_ex101() {
        let input = " ```
 aaa
aaa
```";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
aaa"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }

    #[test]
    fn test_commonmark_ex102() {
        let input = "  ```
aaa
  aaa
aaa
  ```";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
aaa
aaa"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex103() {
        let input = "   ```
   aaa
    aaa
  aaa
   ```";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
 aaa
aaa"
            .to_string(),
        );
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex105() {
        let input = "```
aaa
  ```";
        let want = CodeBlock(Some("".to_string()), "aaa".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex106() {
        let input = "   ```
aaa
  ```";
        let want = CodeBlock(Some("".to_string()), "aaa".to_string());
        let (input, result) = code_block(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(want, result);
    }
    #[test]
    fn test_commonmark_ex107() {
        let input = "```
aaa
    ```";
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
    fn test_commonmark_ex109() {
        let input = "~~~~~~
aaa
~~~ ~~";
        let want = CodeBlock(
            Some("".to_string()),
            "aaa
~~~ ~~"
                .to_string(),
        );
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

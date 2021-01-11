#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Block {
    Header(Vec<Inline>, usize),
    Paragraph(Vec<Inline>),
    Blockquote(Vec<Block>),
    CodeBlock(Option<String>, String),
    LinkRefence(String, String, Option<String>),
    List(Vec<ListItem>, ListType),
    Formula(String),
    HorizontalRule,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListItem {
    TaskListItem(Vec<Block>, bool),
    OrderedListItem(Vec<Block>, usize),
    UnorderedListItem(Vec<Block>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ListType {
    TaskList,
    OrderedList,
    UnorderedList,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Inline {
    Break,
    Text(String),
    Code(String),
    Literal(char),
    Link(Vec<Inline>, String, Option<String>),
    // [text](url "title")
    RefLink(Vec<Inline>, String, String),
    // [alt](url "title")
    Image(String, String, Option<String>),
    Emphasis(Vec<Inline>),
    Strong(Vec<Inline>),
    Formula(String),
    AutoLink(String),
    Strike(Vec<Inline>),
    Subscript(Vec<Inline>),
    Superscript(Vec<Inline>),
    RawHTML(String),
}

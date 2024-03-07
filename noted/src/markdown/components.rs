#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownLine {
    Header(usize, Vec<MarkdownSegment>),
    UnorderedListElement(Vec<MarkdownSegment>),
    OrderedListElement(usize, Vec<MarkdownSegment>),
    Segments(Vec<MarkdownSegment>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownSegment {
    Text(String),
    InlineCode(String),
    Bold(Vec<MarkdownSegment>),
    Italic(Vec<MarkdownSegment>),
    Strikethrough(Vec<MarkdownSegment>),
}
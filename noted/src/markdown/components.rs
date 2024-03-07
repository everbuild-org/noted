#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownComponent {
    Header(usize, Vec<MarkdownComponent>),
    UnorderedListElement(Vec<MarkdownComponent>),
    OrderedListElement(usize, Vec<MarkdownComponent>),
    TextSegment(String),
    InlineCode(String),
    Bold(Vec<MarkdownComponent>),
    Italic(Vec<MarkdownComponent>),
    Strikethrough(Vec<MarkdownComponent>),
}
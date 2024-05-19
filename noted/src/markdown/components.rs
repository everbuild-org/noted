pub struct AnnotatedMarkdownLine {
    pub line: MarkdownLine,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownLine {
    Header(usize, Vec<MarkdownSegment>),
    UnorderedListElement(Vec<MarkdownSegment>),
    OrderedListElement(usize, Vec<MarkdownSegment>),
    Segments(Vec<MarkdownSegment>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MarkdownSegment {
    Text(String, MarkdownSegmentDecoration),
    InlineCode(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MarkdownSegmentDecoration {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
}

impl Default for MarkdownSegmentDecoration {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            strikethrough: false,
        }
    }
}

impl MarkdownSegmentDecoration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }
}

impl MarkdownSegment {
    pub fn split_whitespace(&self) -> Vec<MarkdownSegment> {
        match self {
            MarkdownSegment::Text(data, decoration) => {
                let mut segments = vec![];
                let mut buffer = String::new();
                for c in data.chars() {
                    if "\r\n".contains(c) {
                        continue;
                    }

                    if c.is_whitespace() {
                        if !buffer.is_empty() {
                            segments.push(MarkdownSegment::Text(buffer.clone(), decoration.clone()));
                            buffer.clear();
                        }
                        segments.push(MarkdownSegment::Text(c.to_string(), decoration.clone()));
                    } else {
                        buffer.push(c);
                    }
                }

                if !buffer.is_empty() {
                    segments.push(MarkdownSegment::Text(buffer.clone(), decoration.clone()));
                }
                segments
            }

            // Inline code should not split
            MarkdownSegment::InlineCode(data) => vec![MarkdownSegment::InlineCode(data.to_string())],
        }
    }
}
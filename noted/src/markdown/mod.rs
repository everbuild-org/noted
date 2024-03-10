use crate::markdown::components::MarkdownLine;
pub use self::parser::{parse_text_line, parse_annotated_text_line};

pub mod components;
pub mod parser;

#[cfg(test)]
pub mod tests;

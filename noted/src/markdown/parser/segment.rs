use crate::markdown::components::MarkdownSegment;
use crate::markdown::components::MarkdownSegment::*;

pub fn parse_segment(line: &str) -> Vec<MarkdownSegment> {
    vec![Text(line.to_string())]
}
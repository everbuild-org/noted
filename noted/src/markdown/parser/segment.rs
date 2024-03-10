use crate::markdown::components::{MarkdownSegment, MarkdownSegmentDecoration};
use crate::markdown::components::MarkdownSegment::*;

pub fn parse_segment(line: &str) -> Vec<MarkdownSegment> {
    vec![Text(line.to_string(), MarkdownSegmentDecoration {
        ..Default::default()
    })]
}
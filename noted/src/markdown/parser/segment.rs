use crate::markdown::components::MarkdownSegment::*;
use crate::markdown::components::{MarkdownSegment, MarkdownSegmentDecoration};

pub fn parse_segment(line: &str) -> Vec<MarkdownSegment> {
    let mut segments = vec![];

    let mut start = 0;
    let mut end = 0;
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_strikethrough = false;

    while end < line.len() {
        let c = line.chars().nth(end).unwrap();

        fn segment_done(
            segments: &mut Vec<MarkdownSegment>,
            line: &str,
            start: &mut usize,
            end: &usize,
            in_bold: &bool,
            in_italic: &bool,
            in_strikethrough: &bool,
        ) {
            segments.push(Text(
                line[start.clone()..end.clone()].to_string(),
                MarkdownSegmentDecoration {
                    bold: in_bold.clone(),
                    italic: in_italic.clone(),
                    strikethrough: in_strikethrough.clone(),
                },
            ));

            *start = end.clone();
        }

        match c {
            '*' => {
                if end + 1 < line.len() && line.chars().nth(end + 1).unwrap() == '*' {
                    segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough);
                    in_bold = !in_bold;
                    end += 2;
                    start += 2;
                } else {
                    segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough);
                    in_italic = !in_italic;
                    end += 1;
                    start += 1;
                }
            }

            '~' => {
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough);
                in_strikethrough = !in_strikethrough;
                end += 1;
                start += 1;
            }

            '_' => {
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough);
                in_italic = !in_italic;
                end += 1;
                start += 1;
            }

            '`' => {
                // Parse until end of block (EOB)
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough);
                start += 1;
                end += 1;
                
                while let Some(c) = line.chars().nth(end) {
                    if c == '`' {
                        // EOB
                        break;
                    }

                    end += 1;
                }

                segments.push(InlineCode(line[start.clone()..end.clone()].to_string()));
                if end < line.len() {
                    end += 1
                }
                start = end.clone();
            }

            _ => {
                end += 1;
            }
        }
    }

    if start < end {
        segments.push(Text(
            line[start..end].to_string(),
            MarkdownSegmentDecoration {
                bold: in_bold,
                italic: in_italic,
                strikethrough: in_strikethrough,
            },
        ));
    }

    segments
}

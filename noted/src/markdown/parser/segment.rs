use crate::markdown::components::MarkdownSegment::*;
use crate::markdown::components::{MarkdownSegment, MarkdownSegmentDecoration};

pub fn parse_segment(line: &str) -> Vec<MarkdownSegment> {
    let mut segments = vec![];

    let mut start = 0;
    let mut end = 0;
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_strikethrough = false;
    let mut in_subscript = false;
    let mut in_superscript = false;
    let mut in_tag = false;

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
            in_subscript: &bool,
            in_superscript: &bool,
        ) {
            segments.push(Text(
                line[start.clone()..end.clone()].to_string(),
                MarkdownSegmentDecoration {
                    bold: in_bold.clone(),
                    italic: in_italic.clone(),
                    strikethrough: in_strikethrough.clone(),
                    subscript: in_subscript.clone(),
                    superscript: in_superscript.clone(),
                },
            ));

            *start = end.clone();
        }

        match c {
            '*' => {
                if end + 1 < line.len() && line.chars().nth(end + 1).unwrap() == '*' {
                    segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
                    in_bold = !in_bold;
                    end += 2;
                    start += 2;
                } else {
                    segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
                    in_italic = !in_italic;
                    end += 1;
                    start += 1;
                }
            }

            '~' => {
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
                in_strikethrough = !in_strikethrough;
                end += 1;
                start += 1;
            }

            '_' => {
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
                in_italic = !in_italic;
                end += 1;
                start += 1;
            }

            '`' => {
                // Parse until end of block (EOB)
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
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

            '<' => {
                segment_done(&mut segments, &line, &mut start, &end, &in_bold, &in_italic, &in_strikethrough, &in_subscript, &in_superscript);
                in_tag = true;
                end += 1;
                start += 1;
            }

            '>' => {
                let tag = line[start..end].to_string();
                in_tag = false;

                end += 1;
                start = end.clone();

                let is_closing = tag.starts_with('/');
                let tag = tag.trim_start_matches('/');

                // sub and sup
                if tag == "sub" {
                    if is_closing {
                        in_subscript = false;
                    } else {
                        in_subscript = true;
                    }
                } else if tag == "sup" {
                    if is_closing {
                        in_superscript = false;
                    } else {
                        in_superscript = true;
                    }
                }
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
                subscript: in_subscript,
                superscript: in_superscript,
            },
        ));
    }

    segments
}

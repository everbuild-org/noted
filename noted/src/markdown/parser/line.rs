use crate::markdown::components::{AnnotatedMarkdownLine, MarkdownLine};
use crate::markdown::components::MarkdownLine::*;
use crate::markdown::parser::parse_segment;

fn parse_header(line: &str) -> Option<MarkdownLine> {
    if !line.starts_with("#") {
        return None;
    }

    let mut level = 0;
    let mut i = 0;

    while i < line.len() {
        if line.chars().nth(i) == Some('#') {
            level += 1;
            i += 1;
        } else {
            break;
        }
    }

    if level > 6 {
        return None;
    }

    let text = line[i..].to_string();

    if text.is_empty() {
        None
    } else if text.starts_with(" ") {
        Some(Header(level, parse_segment(text.trim())))
    } else {
        None
    }
}

fn parse_unordered_list_element(line: &str) -> Option<MarkdownLine> {
    if !line.starts_with("- ") {
        return None;
    }

    let text = line[2..].to_string();

    if text.is_empty() {
        None
    } else {
        Some(UnorderedListElement(parse_segment(text.trim())))
    }
}

fn parse_ordered_list_element(line: &str) -> Option<MarkdownLine> {
    let mut i = 0;
    let mut number = String::new();

    while i < line.len() {
        if let Some(c) = line.chars().nth(i) {
            if c.is_digit(10) {
                number.push(c);
                i += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if number.is_empty() {
        return None;
    }

    if i >= line.len() || line.chars().nth(i) != Some('.') {
        return None;
    }

    let text = line[i + 1..].to_string();

    if text.is_empty() {
        None
    } else {
        Some(OrderedListElement(number.parse().ok()?, parse_segment(text.trim())))
    }
}

pub fn parse_text_line(line: &str) -> MarkdownLine {
    if let Some(header) = parse_header(line) {
        return header;
    }

    if let Some(unordered_list_element) = parse_unordered_list_element(line) {
        return unordered_list_element;
    }

    if let Some(ordered_list_element) = parse_ordered_list_element(line) {
        return ordered_list_element;
    }

    Segments(parse_segment(line))
}

pub fn parse_annotated_text_line(source: &str) -> AnnotatedMarkdownLine {
    let line = parse_text_line(source);

    AnnotatedMarkdownLine {
        line,
        source: source.to_string(),
    }
}
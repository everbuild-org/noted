use crate::markdown::components::MarkdownLine;
use crate::markdown::components::MarkdownLine::*;
use crate::markdown::components::MarkdownSegment::*;
use super::*;

fn text_assertion(line: &str, expected: MarkdownLine) {
    let components = parse_text_line(line);
    assert_eq!(components, expected);
}

#[test]
fn test_md_header_1() {
    text_assertion("#  Hello, world!", Header(1, vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_header_3() {
    text_assertion("###  Hello, world!", Header(3, vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_header_7() {
    text_assertion("#######  Hello, world!", Segments(vec![Text("#######  Hello, world!".to_string())]));
}

#[test]
fn test_md_header_nospace() {
    text_assertion("#Hello, world!", Segments(vec![Text("#Hello, world!".to_string())]));
}

#[test]
fn test_md_header_nohash() {
    text_assertion("Hello, world!", Segments(vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_unordered_list() {
    text_assertion("-  Hello, world!", UnorderedListElement(vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_unordered_list_nospace() {
    text_assertion("-Hello, world!", Segments(vec![Text("-Hello, world!".to_string())]));
}

#[test]
fn test_md_ordered_list() {
    text_assertion("1. Hello, world!", OrderedListElement(1, vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_ordered_list_nospace() {
    text_assertion("1.Hello, world!", OrderedListElement(1, vec![Text("Hello, world!".to_string())]));
}

#[test]
fn test_md_ordered_list_no_number() {
    text_assertion(". Hello, world!", Segments(vec![Text(". Hello, world!".to_string())]));
}

#[test]
fn test_md_ordered_list_no_dot() {
    text_assertion("1 Hello, world!", Segments(vec![Text("1 Hello, world!".to_string())]));
}

#[test]
fn test_md_ordered_list_higher_number() {
    text_assertion("100. Hello, world!", OrderedListElement(100, vec![Text("Hello, world!".to_string())]));
    text_assertion("1000. Hello, world!", OrderedListElement(1000, vec![Text("Hello, world!".to_string())]));
}
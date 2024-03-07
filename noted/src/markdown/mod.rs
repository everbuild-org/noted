use self::components::MarkdownComponent;

pub mod components;

fn parse_segment(line: &str) -> MarkdownComponent {
    MarkdownComponent::TextSegment(line.to_string())
}

fn parse_header(line: &str) -> Option<MarkdownComponent> {
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
        Some(MarkdownComponent::Header(level, vec![parse_segment(text.trim())]))
    } else {
        None
    }
}

fn parse_unordered_list_element(line: &str) -> Option<MarkdownComponent> {
    if !line.starts_with("- ") {
        return None;
    }

    let text = line[2..].to_string();

    if text.is_empty() {
        None
    } else {
        Some(MarkdownComponent::UnorderedListElement(vec![parse_segment(text.trim())]))
    }
}

fn parse_ordered_list_element(line: &str) -> Option<MarkdownComponent> {
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
        Some(MarkdownComponent::OrderedListElement(number.parse().ok()?, vec![parse_segment(text.trim())]))
    }
}

pub fn parse_text_line(line: &str) -> MarkdownComponent {
    if let Some(header) = parse_header(line) {
        return header;
    }

    if let Some(unordered_list_element) = parse_unordered_list_element(line) {
        return unordered_list_element;
    }

    if let Some(ordered_list_element) = parse_ordered_list_element(line) {
        return ordered_list_element;
    }

    parse_segment(line)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn text_assertion(line: &str, expected: MarkdownComponent) {
        let components = parse_text_line(line);
        assert_eq!(components, expected);
    }

    #[test]
    fn test_md_header_1() {
        text_assertion("#  Hello, world!", MarkdownComponent::Header(1, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_header_2() {
        text_assertion("##  Hello, world!", MarkdownComponent::Header(2, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_header_3() {
        text_assertion("###  Hello, world!", MarkdownComponent::Header(3, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_header_7() {
        text_assertion("#######  Hello, world!", MarkdownComponent::TextSegment("#######  Hello, world!".to_string()));
    }

    #[test]
    fn test_md_header_nospace() {
        text_assertion("#Hello, world!", MarkdownComponent::TextSegment("#Hello, world!".to_string()));
    }

    #[test]
    fn test_md_header_nohash() {
        text_assertion("Hello, world!", MarkdownComponent::TextSegment("Hello, world!".to_string()));
    }

    #[test]
    fn test_md_unordered_list() {
        text_assertion("-  Hello, world!", MarkdownComponent::UnorderedListElement(vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_unordered_list_nospace() {
        text_assertion("-Hello, world!", MarkdownComponent::TextSegment("-Hello, world!".to_string()));
    }

    #[test]
    fn test_md_ordered_list() {
        text_assertion("1. Hello, world!", MarkdownComponent::OrderedListElement(1, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_ordered_list_nospace() {
        text_assertion("1.Hello, world!", MarkdownComponent::OrderedListElement(1, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }

    #[test]
    fn test_md_ordered_list_no_number() {
        text_assertion(". Hello, world!", MarkdownComponent::TextSegment(". Hello, world!".to_string()));
    }

    #[test]
    fn test_md_ordered_list_no_dot() {
        text_assertion("1 Hello, world!", MarkdownComponent::TextSegment("1 Hello, world!".to_string()));
    }

    #[test]
    fn test_md_ordered_list_higher_number() {
        text_assertion("100. Hello, world!", MarkdownComponent::OrderedListElement(100, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
        text_assertion("1000. Hello, world!", MarkdownComponent::OrderedListElement(1000, vec![MarkdownComponent::TextSegment("Hello, world!".to_string())]));
    }
}
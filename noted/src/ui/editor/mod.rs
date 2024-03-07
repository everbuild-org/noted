use gpui::div;
use gpui::Render;
use gpui::prelude::*;
use gpui::View;
use gpui::ViewContext;
use crate::markdown::parse_annotated_text_line;
use crate::markdown::ui::line::MarkdownLineRenderer;

use super::Shell;

pub struct Editor {

}

impl Editor {
    pub fn build(cx: &mut ViewContext<Shell>) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }

}

impl Render for Editor {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_grow()
            .p_8()
            .children(vec![
                MarkdownLineRenderer::from(parse_annotated_text_line("# Heading 1")),
                MarkdownLineRenderer::from(parse_annotated_text_line("## Heading 2")),
                MarkdownLineRenderer::from(parse_annotated_text_line("### Heading 3")),
                MarkdownLineRenderer::from(parse_annotated_text_line("#### Heading 4")),
                MarkdownLineRenderer::from(parse_annotated_text_line("##### Heading 5")),
                MarkdownLineRenderer::from(parse_annotated_text_line("###### Heading 6")),
                MarkdownLineRenderer::from(parse_annotated_text_line("This is a paragraph of text")),
                MarkdownLineRenderer::from(parse_annotated_text_line("- This is an unordered list 1/2")),
                MarkdownLineRenderer::from(parse_annotated_text_line("- This is an unordered list 2/2")),
                MarkdownLineRenderer::from(parse_annotated_text_line("1. This is an ordered list 1/2")),
                MarkdownLineRenderer::from(parse_annotated_text_line("2. This is an ordered list 2/2")),
                MarkdownLineRenderer::from(parse_annotated_text_line("This is another paragraph of text"))// long enough to wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like soo long that it should wrap around the screen and test the wrapping of the text, like so")),
            ])
    }
}
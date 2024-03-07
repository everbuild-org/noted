use gpui::div;
use gpui::Render;
use gpui::prelude::*;
use gpui::View;
use gpui::ViewContext;
use crate::markdown::parse_text_line;
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
            .children(vec![
                MarkdownLineRenderer::from(parse_text_line("# Hello, world!")),
                MarkdownLineRenderer::from(parse_text_line("## Hello, world!")),
                MarkdownLineRenderer::from(parse_text_line("Hello, world!"))
            ])
    }
}
use gpui::{div, px};
use gpui::Render;
use gpui::prelude::*;
use gpui::View;
use gpui::ViewContext;
use ropey::Rope;
use crate::markdown::parse_annotated_text_line;
use crate::ui::editor::markdown::line::MarkdownLineRenderer;

use super::Shell;

pub mod markdown;

pub struct Editor {
    content: Rope,
}

impl Editor {
    pub fn build(cx: &mut ViewContext<Shell>) -> View<Self> {
        cx.new_view(|_cx| Self {
            content: Rope::from_str(include_str!("../../../../data/welcome.md"))
        })
    }

    pub fn count_characters(&self) -> usize {
        self.content.len_chars()
    }

    pub fn count_words(&self) -> usize {
        // todo make this faster by not allocating a new string the whole length of the content
        self.content.to_string().split_whitespace().count()
    }
}

impl Render for Editor {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_grow()
            .p_8()
            .flex()
            .flex_col()
            .items_start()
            .items_center()
            .child(
                div()
                    .max_w(px(1000f32))
                    .children(self.content.lines().map(|line| {
                        MarkdownLineRenderer::from(parse_annotated_text_line(line.as_str().expect("Failed to parse line")))
                    }))
            )
    }
}

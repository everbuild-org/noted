use gpui::{div, Fill, Hsla, MouseButton, px};
use gpui::Render;
use gpui::prelude::*;
use gpui::View;
use gpui::ViewContext;
use ropey::Rope;
use crate::markdown::parse_annotated_text_line;
use crate::ui::editor::markdown::reading_line::MarkdownLineRenderer;

use super::Shell;

pub mod markdown;

pub enum Selection {
    Pos(usize, usize),
}

pub struct Editor {
    content: Rope,
    selection: Selection,
}

impl Editor {
    pub fn build(cx: &mut ViewContext<Shell>) -> View<Self> {
        cx.new_view(|_cx| Self {
            content: Rope::from_str(include_str!("../../../../data/welcome.md")),
            selection: Selection::Pos(0, 0),
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
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
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
                    .children(self.content.lines().enumerate().map(|(id, line)| {
                        match self.selection {
                            Selection::Pos(selection_line, _) => {
                                if selection_line == id {
                                    div()
                                        .bg(Fill::Color(Hsla::blue()))
                                        .child(MarkdownLineRenderer::from(parse_annotated_text_line(line.as_str().expect("Failed to parse line"))))
                                } else {
                                    div()
                                        .child(MarkdownLineRenderer::from(parse_annotated_text_line(line.as_str().expect("Failed to parse line"))))
                                }.on_mouse_down(MouseButton::Left, cx.listener(move |editor, _, cx| {
                                    editor.selection = Selection::Pos(id, 0);
                                    cx.notify();
                                }))
                            }
                        }
                    }))
            )
    }
}

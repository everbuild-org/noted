use std::iter::{Enumerate, Map};
use crate::markdown::parse_annotated_text_line;
use crate::ui::editor::markdown::reading_line::MarkdownLineRenderer;
use gpui::prelude::*;
use gpui::{Div, FocusHandle, Render, WindowContext};
use gpui::View;
use gpui::ViewContext;
use gpui::{div, px, Fill, Hsla, MouseButton};
use ropey::iter::Lines;
use ropey::{Rope, RopeSlice};
use crate::keymap::{CursorMove, MovementDirection};

use super::Shell;

pub mod markdown;

pub enum Selection {
    Pos(usize, usize),
}

pub struct Editor {
    content: Rope,
    selection: Selection,
    focus_handle: FocusHandle,
}

impl Editor {
    pub fn build(cx: &mut ViewContext<Shell>) -> View<Self> {
        cx.new_view(|cx| Self {
            content: Rope::from_str(include_str!("../../../../data/welcome.md")),
            selection: Selection::Pos(0, 0),
            focus_handle: cx.focus_handle(),
        })
    }

    pub fn focus(&self, cx: &mut WindowContext) {
        self.focus_handle.focus(cx);
    }

    pub fn count_characters(&self) -> usize {
        self.content.len_chars()
    }

    pub fn count_words(&self) -> usize {
        // todo make this faster by not allocating a new string the whole length of the content
        self.content.to_string().split_whitespace().count()
    }

    fn lines(&mut self, cx: &mut ViewContext<Self>) -> Vec<Div> {
        self.content.lines().enumerate().map(|(id, line)| {
            match self.selection {
                Selection::Pos(selection_line, _) => if selection_line == id {
                    div()
                        .bg(Fill::Color(Hsla::blue()))
                        .min_h_5()
                        .child(
                            MarkdownLineRenderer::from(parse_annotated_text_line(
                                line.as_str().expect("Failed to parse line"),
                            )),
                        )
                } else {
                    div()
                        .min_h_5()
                        .child(MarkdownLineRenderer::from(parse_annotated_text_line(
                            line.as_str().expect("Failed to parse line"),
                        )))
                }
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |editor, _, cx| {
                            editor.selection = Selection::Pos(id, 0);
                            cx.notify();
                        }),
                    ),
            }
        }).collect()
    }

    fn move_line(selection: &mut Selection, dx: isize, dy: isize) {
        match selection {
            Selection::Pos(x, y) => {
                let x: isize = x.clone() as isize;
                let y: isize = y.clone() as isize;

                *selection = Selection::Pos((x + dx) as usize, (y + dy) as usize)
            }
        }
    }

    fn movement(&mut self, mv: &CursorMove, cx: &mut ViewContext<Self>) {
        match mv.direction {
            MovementDirection::Up => { Editor::move_line(&mut self.selection, -1, 0) }
            MovementDirection::Down => { Editor::move_line(&mut self.selection, 1, 0) }
            MovementDirection::Left => { Editor::move_line(&mut self.selection, 0, 0) } //TODO
            MovementDirection::Right => { Editor::move_line(&mut self.selection, 0, 0) } // TODO
        }

        cx.refresh();
    }
}

impl Render for Editor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                    .track_focus(&self.focus_handle)
                    .key_context("text")
                    .on_action(cx.listener(Editor::movement))
                    .children(self.lines(cx)),
            )
    }
}

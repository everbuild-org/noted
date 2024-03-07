
use gpui::{Div, div, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, WindowContext};
use gpui::prelude::FluentBuilder;
use crate::markdown::components::{AnnotatedMarkdownLine, MarkdownLine, MarkdownSegment};
use crate::markdown::ui::segment::MarkdownSegmentRenderer;
use crate::theme::Theme;

#[derive(IntoElement)]
pub struct MarkdownLineRenderer {
    pub component: MarkdownLine,
    pub source: String
}

fn segment_children_div(data: &Vec<MarkdownSegment>) -> Div {
    let mut div = div();
    for segment in data {
        div = div.child(MarkdownSegmentRenderer::from(segment.clone()));
    }
    div
}

impl MarkdownLineRenderer {
    pub fn new(component: MarkdownLine, source: String) -> Self {
        Self { component, source }
    }

    fn render_text_line(&self, _cx: &mut WindowContext, data: &Vec<MarkdownSegment>) -> Div {
        segment_children_div(data)
    }

    fn render_heading(&self, _cx: &mut WindowContext, depth: &usize, data: &Vec<MarkdownSegment>) -> Div {
        segment_children_div(data)
            .font_weight(FontWeight::BOLD)
            .when(*depth == 1, |div| div.text_3xl())
            .when(*depth == 2, |div| div.text_2xl())
            .when(*depth == 3, |div| div.text_xl())
            .when(*depth == 4, |div| div.text_lg())
            .when(*depth == 5, |div| div.text_sm())
            .when(*depth == 6, |div| div.text_xs())
    }

    fn render_unordered_list(&self, cx: &mut WindowContext, data: &Vec<MarkdownSegment>) -> Div {
        let theme = cx.global::<Theme>();
        div()
            .flex()
            .flex_row()
            .child(
                div()
                    .child("•")
                    .w_5()
                    .text_color(&theme.foreground.opacity(0.75))
                    .flex()
                    .justify_center()
            )
            .child(segment_children_div(data))
            .flex_grow()
    }

    fn render_ordered_list(&self, cx: &mut WindowContext, pos: &usize, data: &Vec<MarkdownSegment>) -> Div {
        let theme = cx.global::<Theme>();
        div()
            .flex()
            .flex_row()
            .child(
                div()
                    .child(format!("{}. ", pos))
                    .min_w_5() // todo: make this a theme value
                    .text_color(&theme.foreground.opacity(0.75))
                    .flex()
                    .justify_end()
            )
            .child(segment_children_div(data))
            .flex_grow()
    }
}

impl RenderOnce for MarkdownLineRenderer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match &self.component {
            MarkdownLine::Header(depth, data) => self.render_heading(cx, depth, data),
            MarkdownLine::UnorderedListElement(data) => self.render_unordered_list(cx, data),
            MarkdownLine::OrderedListElement(pos, data) => self.render_ordered_list(cx, pos, data),
            MarkdownLine::Segments(data) => self.render_text_line(cx, data)
        }
    }
}

impl From<AnnotatedMarkdownLine> for MarkdownLineRenderer {
    fn from(value: AnnotatedMarkdownLine) -> Self {
        MarkdownLineRenderer::new(value.line, value.source)
    }
}
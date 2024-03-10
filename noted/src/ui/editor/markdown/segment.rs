use gpui::{Div, div, FontStyle, FontWeight, IntoElement, ParentElement, RenderOnce, StrikethroughStyle, Styled, WindowContext};
use gpui::prelude::FluentBuilder;
use crate::markdown::components::{MarkdownSegment, MarkdownSegmentDecoration};

#[derive(IntoElement)]
pub struct MarkdownSegmentRenderer {
    pub component: MarkdownSegment
}

impl MarkdownSegmentRenderer {
    pub fn new(component: MarkdownSegment) -> Self {
        Self { component }
    }
    pub fn from(component: MarkdownSegment) -> Vec<Self> {
        // Split on spaces
        let mut segments = vec![];
        for segment in component.split_whitespace() {
            segments.push(Self::new(segment));
        }
        segments
    }

    fn render_text(&self, cx: &mut WindowContext, data: &str, decoration: &MarkdownSegmentDecoration) -> Div {
        div()
            .when(decoration.bold, |this| this.font_weight(FontWeight::BOLD))
            .when(decoration.italic, |mut this| {
                let refinement = this.text_style().get_or_insert_with(Default::default);
                refinement.font_style = Some(FontStyle::Italic);
                this
            })
            .when(decoration.strikethrough, |mut this| {
                let refinement = this.text_style().get_or_insert_with(Default::default);
                refinement.strikethrough = Some(StrikethroughStyle::default());
                this
            })
            .child(data.to_string())
    }
}

impl RenderOnce for MarkdownSegmentRenderer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match &self.component {
            MarkdownSegment::Text(data, style) => self.render_text(cx, data, style),
            MarkdownSegment::InlineCode(_) => { todo!("Render inline code") }
        }
    }
}

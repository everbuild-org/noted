
use gpui::{Div, div, IntoElement, ParentElement, RenderOnce, WindowContext};
use crate::markdown::components::MarkdownSegment;

#[derive(IntoElement)]
pub struct MarkdownSegmentRenderer {
    pub component: MarkdownSegment
}

impl MarkdownSegmentRenderer {
    pub fn new(component: MarkdownSegment) -> Self {
        Self { component }
    }

    fn render_text(&self, cx: &mut WindowContext, data: &str) -> Div {
        div()
            .child(data.to_string())
    }
}

impl RenderOnce for MarkdownSegmentRenderer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match &self.component {
            MarkdownSegment::Text(data) => self.render_text(cx, data),
            MarkdownSegment::InlineCode(_) => { todo!("Render inline code") }
            MarkdownSegment::Bold(_) => { todo!("Render bold") }
            MarkdownSegment::Italic(_) => { todo!("Render italic") }
            MarkdownSegment::Strikethrough(_) => { todo!("Render strikethrough") }
        }
    }
}

impl From<MarkdownSegment> for MarkdownSegmentRenderer {
    fn from(value: MarkdownSegment) -> Self {
        MarkdownSegmentRenderer::new(value)
    }
}
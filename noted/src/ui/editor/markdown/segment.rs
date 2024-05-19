use gpui::{div, px, Div, Font, FontStyle, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement, RenderOnce, StrikethroughStyle, Styled, WindowContext};
use gpui::prelude::FluentBuilder;
use log::info;
use crate::markdown::components::{MarkdownSegment, MarkdownSegmentDecoration};
use crate::theme::Theme;

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

    fn render_text(&self, _cx: &mut WindowContext, data: &str, decoration: &MarkdownSegmentDecoration) -> Div {
        let deco = decoration.clone();
        div()
            .when(decoration.bold, |this| this.font_weight(FontWeight::BOLD))
            .when(decoration.italic, |mut this| {
                let refinement = this.text_style().get_or_insert_with(Default::default);
                refinement.font_style = Some(FontStyle::Italic);
                this
            })
            .when(/*decoration.strikethrough*/ true, |mut this| {
                let refinement = this.text_style().get_or_insert_with(Default::default);
                refinement.strikethrough = Some(StrikethroughStyle { thickness: px(1f32), color: Some(Hsla::white()) });
                this
            })
            .whitespace_normal()
            .child(data.to_string())
            .hover(|s| s.debug())
            .on_mouse_down(gpui::MouseButton::Left, move |_e, _cx| {
                info!("{:?}", deco);
            })
    }

    fn render_code(&self, cx: &WindowContext, data: &String) -> Div {
        let theme: &Theme = cx.global();
        div()
            .font("Zed Mono")
            .p_0p5()
            .pl_1()
            .rounded_md()
            .bg(theme.background_secondary.fill())
            .child(data.clone())
            .hover(|s|s.debug())
    }
}

impl RenderOnce for MarkdownSegmentRenderer {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        match &self.component {
            MarkdownSegment::Text(data, style) => self.render_text(cx, data, style),
            MarkdownSegment::InlineCode(data) => { self.render_code(cx, data) }
        }
    }
}

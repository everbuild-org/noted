use gpui::{div, IntoElement, Model, ParentElement, px, relative, Render, Styled, ViewContext};
use lucide_gpui::Icon;
use crate::BaseModel;

pub struct Shell {
    pub(crate) model: Model<BaseModel>,
}

impl Render for Shell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.model.read(cx).theme.borrow();

        div()
            .h(relative(1.0))
            .flex()
            .children(vec![
                div()
                    .h(relative(1.0))
                    .w(relative(1.0))
                    .flex()
                    .flex_col()
                    .children(vec![
                        div()
                            .flex_grow()
                            .child(
                                div()
                                    .child("Editor")
                                    .text_color(&theme.foreground)
                            ),
                        div()
                            .h(px(35.0))
                            .text_size(px(14.0))
                            .bg(theme.background_highlight.fill())
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_2()
                            .children(
                                vec![
                                    div()
                                        .text_color(&theme.foreground.opacity(0.5))
                                        .child(Icon::icon_a_arrow_down()),
                                    div()
                                        .text_color(&theme.foreground)
                                        .flex()
                                        .gap_2()
                                        .children(vec![
                                            div()
                                                .text_color(&theme.foreground.opacity(0.5))
                                                .child("Right Sidebar"),
                                            div()
                                                .text_color(&theme.foreground.opacity(0.25))
                                                .child("|"),
                                            div()
                                                .text_color(&theme.foreground.opacity(0.5))
                                                .child("Right Sidebar Toggle"),
                                        ]),
                                ]
                            ),
                    ])
            ])
    }
}
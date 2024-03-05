use gpui::{Context, div, IntoElement, Model, ParentElement, px, Render, Styled, ViewContext};
use crate::icon::{AppIconSize, Icon};
use crate::pane::Panes;
use crate::theme::Theme;
use crate::ui::components::icon_toggle::icon_toggle_button;

pub struct StatusBar {
    panes: Model<Panes>,
}

impl StatusBar {
    pub fn new(_ctx: &mut ViewContext<StatusBar>, panes: Model<Panes>) -> Self {
        Self {
            panes,
        }
    }
}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let open_panes = cx.model().to_owned();

        println!("StatusBar render, pane_files: {:?}", self.panes.read(cx).files);

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
                    div().child(
                        icon_toggle_button(self.panes.read(cx).files)
                            .icon(
                                Icon::icon_folder()
                                    .size(AppIconSize::StatusBar)
                            )
                            .on_toggle(move |state, cx|
                                cx.window_handle().downcast::<StatusBar>().unwrap().update(cx, |model| {
                                    model.panes.update(cx, |model| {
                                        model.files = state;
                                    });
                                })
                            )
                    ),
                    div()
                        .text_color(&theme.foreground)
                        .flex()
                        .gap_2()
                        .items_center()
                        .children(vec![
                            div()
                                .text_color(&theme.foreground.opacity(0.5))
                                .child("info bar"),
                            div()
                                .text_color(&theme.foreground.opacity(0.25))
                                .child("|"),
                            div()
                                .child(
                                    icon_toggle_button(self.pane_files.clone())
                                        .icon(
                                            Icon::icon_scatter_chart()
                                                .size(AppIconSize::StatusBar)
                                        )
                                ),
                        ]),
                ]
            )
    }
}

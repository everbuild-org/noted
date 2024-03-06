use crate::icon::{AppIconSize, Icon};
use crate::pane::{PaneToggle, Panes};
use crate::theme::Theme;
use crate::ui::components::clickable::clickable;
use crate::ui::components::icon_toggle::icon_toggle_button;
use gpui::{
    div, px, EventEmitter, IntoElement, Model, ParentElement, Render, Styled, ViewContext,
    WindowContext,
};

pub struct StatusBar {
    panes: Model<Panes>,
}

impl StatusBar {
    pub fn new(_ctx: &mut ViewContext<StatusBar>, panes: &Model<Panes>) -> Self {
        Self {
            panes: panes.clone(),
        }
    }
}

impl EventEmitter<PaneToggle> for StatusBar {}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        println!(
            "StatusBar render, pane_files: {:?}",
            self.panes.read(cx).files
        );

        div()
            .h(px(35.0))
            .text_size(px(14.0))
            .bg(theme.background_highlight.fill())
            .flex()
            .items_center()
            .justify_between()
            .px_2()
            .child(
                div().child(
                    icon_toggle_button(self.panes.read(cx).files)
                        .icon(Icon::icon_folder().size(AppIconSize::StatusBar))
                        .on_toggle({
                            let view = cx.view().downgrade().upgrade().unwrap();
                            move |e: &bool, cx: &mut WindowContext| {
                                println!("^1 StatusBar icon_toggle_button on_toggle");
                                view.update(cx, |view, cx| {
                                    println!("^2 StatusBar icon_toggle_button on_toggle");
                                    cx.emit(PaneToggle::Files(*e));
                                    cx.notify();
                                });
                            }
                        }),
                ),
            )
            .child(
                div()
                    .text_color(&theme.foreground)
                    .flex()
                    .gap_2()
                    .items_center()
                    .children(vec![
                        div()
                            .text_color(&theme.foreground.opacity(0.5))
                            .child("info bar"),
                        div().text_color(&theme.foreground.opacity(0.25)).child("|"),
                        div().child(
                            icon_toggle_button(self.panes.read(cx).files)
                                .icon(Icon::icon_scatter_chart().size(AppIconSize::StatusBar)),
                        ),
                    ]),
            )
    }
}

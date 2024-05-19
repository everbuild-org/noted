use crate::icon::{AppIconSize, Icon};
use crate::pane::{PaneToggle, Panes};
use crate::theme::Theme;
use crate::ui::components::icon_toggle::icon_toggle_button;
use gpui::{div, px, EventEmitter, IntoElement, Model, ParentElement, Render, Styled, ViewContext, View};
use crate::ui::editor::Editor;
use crate::ui::information_bar::InformationBar;

pub struct StatusBar {
    panes: Model<Panes>,
    editor: View<Editor>
}

impl StatusBar {
    pub fn new(_ctx: &mut ViewContext<StatusBar>, panes: &Model<Panes>, editor: &View<Editor>) -> Self {
        Self {
            panes: panes.clone(),
            editor: editor.clone()
        }
    }
}

impl EventEmitter<PaneToggle> for StatusBar {}

impl Render for StatusBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .h(px(35.0))
            .text_size(px(14.0))
            .bg(theme.background_highlight.fill())
            .flex()
            .items_center()
            .justify_between()
            .px_2()
            .child(
                icon_toggle_button(self.panes.read(cx).files)
                    .icon(Icon::icon_folder().size(AppIconSize::StatusBar))
                    .on_toggle(cx.listener(move |_this, e, cx| {
                        cx.emit(PaneToggle::Files(*e));
                    })),
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
                            .child(InformationBar(self.editor.clone())),
                        div().text_color(&theme.foreground.opacity(0.25)).child("|"),
                        div().child(
                            icon_toggle_button(self.panes.read(cx).graph)
                                .icon(Icon::icon_scatter_chart().size(AppIconSize::StatusBar))
                                .on_toggle(cx.listener(move |_this, e, cx| {
                                    cx.emit(PaneToggle::Graph(*e));
                                })),
                        ),
                    ]),
            )
    }
}

mod status_bar;
mod components;

use gpui::{div, relative, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext};
use crate::pane::{PaneToggle, Panes};
use crate::theme::Theme;
use crate::{Noted, VaultReference};

use self::status_bar::StatusBar;

pub struct Shell {
    pub(crate) _vault: Model<VaultReference>,
    pub(crate) status_bar: View<StatusBar>,
}

impl Shell {
    pub fn build(cx: &mut ViewContext<Noted>, base: VaultReference) -> View<Self> {
        let view = cx.new_view(|cx| {
            let vault = cx.new_model(|_cx| base);
            let panes = cx.new_model(|_cx| Panes::default());

            let status_bar = cx.new_view(|cx| StatusBar::new(cx, &panes));
            cx.subscribe(&status_bar, move |_shell, _bar, event, cx| {
                match event {
                    PaneToggle::Files(value) => {
                        cx.update_model(&panes, |model, cx| {
                            model.files = *value;
                            cx.notify();
                        });
                    },
                    PaneToggle::Graph(value) => {
                        cx.update_model(&panes, |model, cx| {
                            model.graph = *value;
                            cx.notify();
                        });
                    },
                }
            }).detach();


            Self {
                _vault: vault,
                status_bar,
            }
        });

        view
    }
}

impl Render for Shell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        let status_bar = self.status_bar.clone();

        div()
            .h(relative(1.0))
            .flex()
            .children(vec![
                div()
                    .h(relative(1.0))
                    .w(relative(1.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .flex_grow()
                            .child(
                                div()
                                    .child("Editor")
                                    .text_color(&theme.foreground)
                            ),
                    )
                    .child(status_bar)
            ])
    }
}

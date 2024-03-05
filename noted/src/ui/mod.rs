mod status_bar;
mod components;

use gpui::{Context, div, IntoElement, Model, ModelContext, ParentElement, relative, Render, Styled, ViewContext, VisualContext};
use crate::BaseModel;
use crate::pane::{Panes, PaneToggle};
use crate::theme::Theme;
use crate::ui::status_bar::StatusBar;

pub struct Shell {
    pub(crate) model: Model<BaseModel>,
}

impl Render for Shell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let panes = cx.new_model(|_| Panes::default());
        cx.new_model(|cx: &mut ModelContext<Panes>| {
            cx.subscribe(&panes, |subscriber, _emitter, event, _cx| {
                match event {
                    PaneToggle::Files(target) => subscriber.files = target.clone(),
                }
            })
                .detach();

            Panes {
                files: panes.read(cx).files.clone(),
            }
        });

        let status_bar = cx.new_view(|ctx| StatusBar::new(ctx, panes));
        let theme = cx.global::<Theme>();

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

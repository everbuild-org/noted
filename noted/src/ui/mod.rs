mod components;
mod file_explorer;
mod status_bar;
mod editor;

use crate::pane::{PaneToggle, Panes};
use crate::ui::file_explorer::FileExplorerPane;
use crate::{Noted, VaultReference};
use gpui::prelude::FluentBuilder;
use gpui::{div, relative, Context, IntoElement, Model, ParentElement, Render, Styled, View, ViewContext, VisualContext, EventEmitter};

use self::editor::Editor;
use self::status_bar::StatusBar;

pub struct Shell {
    pub(crate) _vault: Model<VaultReference>,
    pub(crate) panes: Model<Panes>,
    pub(crate) status_bar: View<StatusBar>,
    pub(crate) file_explorer: View<FileExplorerPane>,
    pub(crate) editor: View<Editor>,
}

impl Shell {
    pub fn build(cx: &mut ViewContext<Noted>, base: VaultReference) -> View<Self> {
        let view = cx.new_view(|cx| {
            let vault = cx.new_model(|_cx| base);
            let panes = cx.new_model(|_cx| Panes::default());

            let panes_for_status_bar = panes.clone();
            let status_bar = cx.new_view(|cx| StatusBar::new(cx, &panes_for_status_bar));

            let file_explorer = cx.new_view(|cx| FileExplorerPane::new(cx));
            let editor = Editor::build(cx);

            Self::subscribe_to_pane_toggle(panes.clone(), &status_bar, cx);
            Self::subscribe_to_pane_toggle(panes.clone(), &file_explorer, cx);

            Self {
                _vault: vault,
                status_bar,
                file_explorer,
                panes,
                editor,
            }
        });

        view
    }

    fn subscribe_to_pane_toggle<T : EventEmitter<PaneToggle>>(panes: Model<Panes>, view: &View<T>, cx: &mut ViewContext<Self>) {
        cx.subscribe(view, move |_shell, _panes, event, cx| match event {
            PaneToggle::Files(value) => {
                cx.update_model(&panes, |model, cx| {
                    model.files = *value;
                    cx.notify();
                });
            }
            PaneToggle::Graph(value) => {
                cx.update_model(&panes, |model, cx| {
                    model.graph = *value;
                    cx.notify();
                });
            }
        })
        .detach();
    }
}

impl Render for Shell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let status_bar = self.status_bar.clone();

        div().h(relative(1.0)).flex().children(vec![div()
            .h(relative(1.0))
            .w(relative(1.0))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_row()
                    .when(self.panes.read(cx).files, |cx| {
                        cx.child(self.file_explorer.clone())
                    })
                    .child(self.editor.clone()),
            )
            .child(status_bar)])
    }
}

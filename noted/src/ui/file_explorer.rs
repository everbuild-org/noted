use gpui::{div, IntoElement, ParentElement, Pixels, px, Render, Styled, View, ViewContext, VisualContext};
use crate::theme::Theme;
use crate::ui::components::drag_handle::{drag_handle, DragEvent, DragHandle, DragHandleDirection};

pub struct FileExplorerPane {
    handle: View<DragHandle>,
    transaction_width: Pixels,
}

impl FileExplorerPane {
    pub fn new(cx: &mut ViewContext<FileExplorerPane>) -> Self {
        let handle = cx.new_view(|cx|
            drag_handle(
                DragHandleDirection::Vertical,
                cx.focus_handle(),
                "file_explorer_pane_drag_handle",
            )
        );

        cx.subscribe(&handle, move |this, _handle, event, _cx| {
            match event {
                DragEvent::Move { rel } => {
                    this.transaction_width = this.transaction_width + rel.clone();
                    if this.transaction_width < px(200f32) {
                        this.transaction_width = px(200f32);
                    }
                }
            }
        }).detach();

        Self {
            handle,
            transaction_width: Pixels(300.0),
        }
    }
}

impl Render for FileExplorerPane {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let handle = self.handle.clone();
        div()
            .bg(theme.background_secondary.fill())
            .w(self.transaction_width)
            .h_full()
            .flex()
            .flex_row()
            .child(div().flex_grow())
            .child(handle)
    }
}
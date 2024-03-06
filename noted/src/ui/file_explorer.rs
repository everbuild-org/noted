use gpui::{div, DragMoveEvent, Fill, Hsla, IntoElement, ParentElement, Render, Styled, View, ViewContext, VisualContext};
use log::info;
use crate::ui::components::drag_handle::{drag_handle, DragHandle, DragHandleDirection};

pub struct FileExplorerPane(View<DragHandle>);

impl FileExplorerPane {
    pub fn new(cx: &mut ViewContext<FileExplorerPane>) -> Self {
        Self(
            cx.new_view(|cx|
                drag_handle(DragHandleDirection::Vertical, cx.focus_handle(), "file_explorer_pane_drag_handle")
                    .on_drag(|drag, cx| {
                        info!("drag status: {:?}", drag.event);
                    })
            )
        )
    }
}

impl Render for FileExplorerPane {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let handle = self.0.clone();
        div()
            .bg(Fill::Color(Hsla::red()))
            .w_72()
            .h_full()
            .flex()
            .flex_row()
            .child(div().flex_grow())
            .child(handle)
    }
}
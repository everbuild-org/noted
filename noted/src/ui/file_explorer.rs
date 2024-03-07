use gpui::{div, EventEmitter, IntoElement, ParentElement, Pixels, px, Render, Styled, View, ViewContext, VisualContext};
use crate::pane::{Panes, PaneToggle};
use crate::theme::Theme;
use crate::ui::components::drag_handle::{drag_handle, DragHandle, DragHandleDirection};

pub struct FileExplorerPane {
    handle: View<DragHandle>,
    width: Pixels,
    negative_px: Pixels,
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

        cx.subscribe(&handle, move |this, _handle, event, cx| {
            this.width = this.width + event.rel.clone();
            if this.width <= px(150f32) {
                this.width = px(150f32);
                this.negative_px += event.rel.clone();

                if this.negative_px < px(-100f32) {
                    cx.emit(PaneToggle::Files(false))
                }
            } else if this.negative_px < px(0f32) {
                this.negative_px = px(0f32);
            }
        }).detach();

        Self {
            handle,
            width: Pixels(300.0),
            negative_px: Pixels(0.0),
        }
    }
}

impl EventEmitter<PaneToggle> for FileExplorerPane {}

impl Render for FileExplorerPane {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let handle = self.handle.clone();
        div()
            .bg(theme.background_secondary.fill())
            .w(self.width)
            .h_full()
            .flex()
            .flex_row()
            .child(div().flex_grow())
            .child(handle)
    }
}
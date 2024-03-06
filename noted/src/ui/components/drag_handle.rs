use gpui::prelude::FluentBuilder;
use gpui::{div, DragMoveEvent, Fill, Hsla, InteractiveElement, StatefulInteractiveElement, IntoElement, Render, RenderOnce, Styled, VisualContext, WindowContext, FocusableView, AppContext, FocusHandle};
use log::info;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragHandleDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Render)]
pub struct Draggable;

pub struct DragHandle {
    pub direction: DragHandleDirection,
    pub callback: Option<Box<dyn Fn(&DragMoveEvent<Draggable>, &mut WindowContext) + 'static>>,
    pub focus_handle: FocusHandle,
    pub id: &'static str,
}

impl DragHandle {
    pub fn on_drag(
        mut self,
        on_drag: impl Fn(&DragMoveEvent<Draggable>, &mut WindowContext) + 'static,
    ) -> Self {
        self.callback = Some(Box::new(on_drag));
        self
    }

    pub fn drag(&self, drag: &DragMoveEvent<Draggable>, cx: &mut WindowContext) {
        if let Some(on_drag) = &self.callback {
            on_drag(drag, cx);
        }
    }
}

impl FocusableView for DragHandle {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DragHandle {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let id: &'static str = self.id;
        div()
            .bg(Fill::Color(Hsla::green()))
            .when(self.direction == DragHandleDirection::Horizontal, |d| {
                d.w_full().h_4()
            })
            .when(self.direction == DragHandleDirection::Vertical, |d| {
                d.w_4().h_full()
            })
            .cursor_grab()
            .id(id)
            .on_drag(Draggable, |drag, cx| {
                cx.stop_propagation();
                cx.new_view(|_| drag.clone())
            })
            .on_drag_move(cx.listener(move |this, drag: &DragMoveEvent<Draggable>, cx| {
                this.drag(drag, cx);
            }))
    }
}

pub fn drag_handle(drag_handle_direction: DragHandleDirection, handle: FocusHandle, id: &'static str) -> DragHandle {
    DragHandle {
        direction: drag_handle_direction,
        callback: None,
        focus_handle: handle,
        id,
    }
}

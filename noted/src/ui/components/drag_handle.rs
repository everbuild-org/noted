use gpui::prelude::FluentBuilder;
use gpui::{div, DragMoveEvent, Fill, Hsla, InteractiveElement, StatefulInteractiveElement, IntoElement, Render, RenderOnce, Styled, VisualContext, WindowContext, FocusableView, AppContext, FocusHandle, EventEmitter, MouseButton};
use crate::prelude::*;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragHandleDirection {
    Horizontal,
    Vertical,
}

pub enum DragEvent {
    Move { rel: Pixels }
}

#[derive(Clone, Render)]
pub struct Draggable(pub DragHandleDirection);

pub struct DragHandle {
    pub direction: DragHandleDirection,
    pub focus_handle: FocusHandle,
    pub id: &'static str,
    pub last_pos: Option<Pixels>,
}

impl FocusableView for DragHandle {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl DragHandle {
    fn drag_line(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .border_color(&theme.ui_line)
            .ml_auto()
            .when(self.direction == DragHandleDirection::Horizontal, |d| {
                d.w_full().border_t_1()
            })
            .when(self.direction == DragHandleDirection::Vertical, |d| {
                d.border_r_1().h_full()
            })
    }
}

impl Render for DragHandle {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let id: &'static str = self.id;
        div()
            .when(self.direction == DragHandleDirection::Horizontal, |d| {
                d.w_full().h_2()
            })
            .when(self.direction == DragHandleDirection::Vertical, |d| {
                d.w_2().h_full()
            })
            .cursor_grab()
            .id(id)
            .on_drag(Draggable(self.direction), {
                let weak = cx.view().downgrade();
                move |drag, cx| {
                    cx.stop_propagation();

                    let view_cx = match weak.upgrade() {
                        Some(view_cx) => view_cx,
                        None => return cx.new_view(|_| drag.clone()),
                    };

                    let mouse = cx.mouse_position();
                    let mouse = match drag.0 {
                        DragHandleDirection::Horizontal => mouse.y,
                        DragHandleDirection::Vertical => mouse.x,
                    };

                    view_cx.model.update(cx, |this, _cx| {
                        this.last_pos = Some(mouse);
                    });

                    cx.new_view(|_| drag.clone())
                }
            })
            .on_drag_move(cx.listener(move |this, drag: &DragMoveEvent<Draggable>, cx| {
                let new_mouse_pos = match this.direction {
                    DragHandleDirection::Horizontal => drag.event.position.y,
                    DragHandleDirection::Vertical => drag.event.position.x,
                };

                let last_pos = this.last_pos.unwrap_or(new_mouse_pos);
                let new_size = new_mouse_pos - last_pos;
                this.last_pos = Some(new_mouse_pos);

                cx.emit(DragEvent::Move { rel: new_size });
            }))
            .child(self.drag_line(cx))
    }
}

impl EventEmitter<DragEvent> for DragHandle {
}

pub fn drag_handle(drag_handle_direction: DragHandleDirection, handle: FocusHandle, id: &'static str) -> DragHandle {
    DragHandle {
        direction: drag_handle_direction,
        focus_handle: handle,
        id,
        last_pos: None,
    }
}

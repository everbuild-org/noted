use gpui::{AnyElement, IntoElement, RenderOnce, WindowContext};
use crate::ui::components::clickable::clickable;


#[derive(IntoElement)]
pub struct ToggleButton {
    element: Option<Box<dyn Fn(bool) -> AnyElement>>,
    toggle_state: bool,
    toggle_callback: Option<Box<dyn Fn(&bool, &mut WindowContext)>>,
}

impl ToggleButton {
    pub fn new(current: bool) -> Self {
        Self {
            element: None,
            toggle_state: current,
            toggle_callback: None,
        }
    }

    fn toggle(&self, cx: &mut WindowContext) {
        let new = !self.toggle_state;
        if let Some(callback) = &self.toggle_callback {
            callback(&new, cx);
        }
    }

    pub(crate) fn child(mut self, element: impl Fn(bool) -> AnyElement + 'static) -> Self {
        self.element = Some(Box::new(element));
        self
    }

    pub fn on_toggle(&mut self, callback: impl Fn(&bool, &mut WindowContext) + 'static) -> &mut Self {
        self.toggle_callback = Some(Box::new(callback));

        self
    }
}

impl RenderOnce for ToggleButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let toggle_state = self.toggle_state.clone();

        clickable()
            .child(
                self.element.as_ref().expect("A toggle button needs some kind of rendered element")(toggle_state))
            .on_click(move |_, cx| self.toggle(cx))
    }
}

pub fn toggle_button(current_state: bool) -> ToggleButton {
    ToggleButton::new(current_state)
}
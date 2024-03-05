use gpui::{div, Div, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, RenderOnce, WindowContext};

#[derive(IntoElement)]
pub struct Clickable {
    callback: Option<Box<dyn Fn(&MouseDownEvent, &mut WindowContext)>>,
    base: Option<Div>,
}

impl Clickable {
    pub fn new() -> Self {
        Self {
            callback: None,
            base: None,
        }
    }

    pub fn on_click(mut self, callback: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.base = Some(div().child(child));
        self
    }

    pub fn base(mut self, base: Div) -> Self {
        self.base = Some(base);
        self
    }
}

pub fn clickable() -> Clickable {
    Clickable::new()
}

impl RenderOnce for Clickable {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let mut base = self.base.expect("Clickable must have a base or child");
        if let Some(callback) = self.callback {
            base = base.on_mouse_down(MouseButton::Left, callback);
        }
        base
    }
}

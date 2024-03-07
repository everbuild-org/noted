use gpui::div;
use gpui::Render;
use gpui::prelude::*;
use gpui::View;
use gpui::ViewContext;

use super::Shell;

pub struct Editor {

}

impl Editor {
    pub fn build(cx: &mut ViewContext<Shell>) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }

}

impl Render for Editor {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .flex_grow()
            .children(vec![div()])
    }
}
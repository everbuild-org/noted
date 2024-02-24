use gpui::{div, IntoElement, Model, ParentElement, relative, Render, Styled, ViewContext};
use crate::BaseModel;

pub struct Shell {
    pub(crate) model: Model<BaseModel>
}

impl Render for Shell {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.model.read(cx).theme.borrow();

        div()
            .w(relative(0.5))
            .h(relative(0.5))
            .bg(theme.primary.fill())
            .text_color(&theme.background)
            .p_1()
            .rounded_md()
            .child("tetris time!")
    }
}
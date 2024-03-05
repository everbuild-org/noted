use gpui::{IntoElement, RenderOnce, WindowContext};
use crate::icon::Icon;
use crate::theme::Theme;
use crate::ui::components::toggle::ToggleButton;

#[derive(IntoElement)]
pub struct IconToggleButton {
    toggle: ToggleButton,
    icon: Option<Icon>
}

impl IconToggleButton {
    pub fn new(current: bool) -> Self {
        Self {
            toggle: ToggleButton::new(current),
            icon: None
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_toggle(mut self, callback: impl Fn(bool, &mut WindowContext) + 'static) -> Self {
        self.toggle.on_toggle(callback);
        self
    }
}

impl RenderOnce for IconToggleButton {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();
        let icon = self.icon.clone().expect("An icon toggle button needs an icon");

        self.toggle
            .child(move |state| {
                icon.color(&theme.foreground.toggled_opacity(1.0, 0.5, state))
                    .into_any_element()
            })
    }
}

pub fn icon_toggle_button(current: bool) -> IconToggleButton {
    IconToggleButton::new(current)
}

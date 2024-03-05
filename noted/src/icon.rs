use gpui::{DefiniteLength, div, Div, IntoElement, Length, ParentElement, RenderOnce, Styled, svg, WindowContext};
use lucide_gpui_macros::{lucide_init, match_icons};
use std::borrow::Cow;


pub fn asset_load_hook(path: &str) -> Option<anyhow::Result<Cow<'static, [u8]>>> {
    if !path.starts_with("lucide:") {
        return None;
    }

    let path = &path[7..];

    return Some(match_icons!(path));
}

#[derive(Clone, Copy)]
pub enum IconSize {
    Xs,
    Sm,
    Md,
    Lg,

    Auto,
    Fraction(f32),
    Px(f32),
    Rem(f32),
}

pub enum AppIconSize {
    StatusBar
}

impl Into<IconSize> for AppIconSize {
    fn into(self) -> IconSize {
        match self {
            AppIconSize::StatusBar => IconSize::Px(16.0),
        }
    }
}

#[derive(Clone, Copy, IntoElement)]
pub struct Icon {
    source: &'static str,
    size: IconSize,
    color: Option<gpui::Hsla>,
}

impl Icon {
    pub fn size(mut self, size: impl Into<IconSize>) -> Self {
        self.size = size.into();
        self
    }

    pub fn color(mut self, color: impl Into<gpui::Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

lucide_init!(Icon);

impl Icon {
    pub fn render_once(self) -> Div {
        let el = div()
            .overflow_hidden()
            .flex()
            .items_center()
            .justify_center();

        let el = match self.size {
            IconSize::Xs => el.size_4(),
            IconSize::Sm => el.size_5(),
            IconSize::Md => el.size_6(),
            IconSize::Lg => el.size_8(),
            IconSize::Auto => el.size_auto(),
            IconSize::Fraction(fraction) => el.size(Length::Definite(DefiniteLength::Fraction(fraction))),
            IconSize::Px(px) => el.size(gpui::px(px)),
            IconSize::Rem(rem) => el.size(gpui::rems(rem)),
        };

        let svg = svg()
            .path(self.source)
            .text_color(self.color.unwrap_or_else(|| gpui::Hsla::white()))
            .size_full();

        el.child(svg.into_any_element())
    }
}

impl RenderOnce for Icon {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        self.render_once()
    }
}
mod included_color_schemes;

use std::fmt::{Debug, Formatter};
use gpui::{hsla, Hsla, rgba, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConfigTheme {
    #[default]
    Dark,
    Custom(Theme),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hsl(u8, u8, u8),
    Hsla(u8, u8, u8, u8),
}

impl Color {
    fn rgba_to_hsla(rgba: &Rgba) -> Hsla {
        let r = rgba.r as f64 * 255.0;
        let g = rgba.g as f64 * 255.0;
        let b = rgba.b as f64 * 255.0;
        let a = rgba.a as f64;

        let cs_rgba = colorsys::Rgb::new(r, g, b, Some(a));
        let cs_hsla: colorsys::Hsl = (&cs_rgba).into();
        let [h, s, l, a] = cs_hsla.into();

        Hsla {
            h: h / 360.0,
            s: s / 100.0,
            l: l / 100.0,
            a,
        }
    }

    pub fn fill(&self) -> gpui::Fill {
        let hsla: Hsla = self.clone().into();
        gpui::Fill::Color(hsla)
    }

    pub fn hsla(&self) -> Hsla {
        self.clone().into()
    }

    pub fn rgba(&self) -> Rgba {
        self.clone().into()
    }

    fn rgba_components(&self) -> (u8, u8, u8, f32) {
        let rgba: Rgba = self.clone().into();
        ((rgba.r * 255.0) as u8, (rgba.g * 255.0) as u8, (rgba.b * 255.0) as u8, rgba.a)
    }

    pub fn opacity(&self, opacity: f32) -> Self {
        let (r, g, b, a) = self.rgba_components();
        Color::Rgba(r, g, b, (a * opacity) as u8)
    }
}

impl Into<Rgba> for Color {
    fn into(self) -> Rgba {
        match self {
            Color::Hex(hex) => {
                Rgba::try_from(hex.as_str()).unwrap_or_else(|_| rgba(0xff_ff_ff_ff))
            },
            Color::Rgb(r, g, b) => {
                let hex = (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | 0xff;
                rgba(hex)
            },
            Color::Rgba(r, g, b, a) => {
                let hex = (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32);
                rgba(hex)
            },
            Color::Hsl(h, s, l) => {
                hsla(
                    h as f32 / 360.0,
                    s as f32 / 100.0,
                    l as f32 / 100.0,
                    1.0,
                ).to_rgb()
            },
            Color::Hsla(h, s, l, a) => {
                hsla(
                    h as f32 / 360.0,
                    s as f32 / 100.0,
                    l as f32 / 100.0,
                    a as f32 / 100.0,
                ).to_rgb()
            },
        }
    }
}

impl Into<Hsla> for Color {
    fn into(self) -> Hsla {
        let rgba: Rgba = self.into();
        Color::rgba_to_hsla(&rgba)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color = self.clone().fill();
        write!(f, "{:?}", color)
    }
}

impl Into<Hsla> for &Color {
    fn into(self) -> Hsla {
        let rgba: Rgba = self.clone().into();
        Color::rgba_to_hsla(&rgba)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
}

impl Into<Theme> for ConfigTheme {
    fn into(self) -> Theme {
        match self {
            ConfigTheme::Dark => included_color_schemes::dark(),
            ConfigTheme::Custom(theme) => theme,
        }
    }
}
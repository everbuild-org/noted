use crate::theme::Color::Hex;
use crate::theme::Theme;

/// Based on the wonderful poimandres theme
pub(crate) fn dark() -> Theme {
    return Theme {
        background: Hex("#1b1e28".to_string()),
        foreground: Hex("#ffffff".to_string()),
        primary: Hex("#5fb3a1".to_string()),
        secondary: Hex("#91b4d5".to_string()),
        accent: Hex("#f087bd".to_string()),
    }
}
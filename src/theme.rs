use iced::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NotedTheme {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
}

impl Into<Theme> for NotedTheme {
    fn into(self) -> Theme {
        match self {
            NotedTheme::Light => Theme::Light,
            NotedTheme::Dark => Theme::Dark,
            NotedTheme::Dracula => Theme::Dracula,
            NotedTheme::Nord => Theme::Nord,
            NotedTheme::SolarizedLight => Theme::SolarizedLight,
            NotedTheme::SolarizedDark => Theme::SolarizedDark,
            NotedTheme::GruvboxLight => Theme::GruvboxLight,
            NotedTheme::GruvboxDark => Theme::GruvboxDark,
            NotedTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            NotedTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            NotedTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            NotedTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            NotedTheme::TokyoNight => Theme::TokyoNight,
            NotedTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            NotedTheme::TokyoNightLight => Theme::TokyoNightLight,
            NotedTheme::KanagawaWave => Theme::KanagawaWave,
            NotedTheme::KanagawaDragon => Theme::KanagawaDragon,
            NotedTheme::KanagawaLotus => Theme::KanagawaLotus,
            NotedTheme::Moonfly => Theme::Moonfly,
            NotedTheme::Nightfly => Theme::Nightfly,
            NotedTheme::Oxocarbon => Theme::Oxocarbon,
        }
    }
}

impl Default for NotedTheme {
    fn default() -> Self {
        NotedTheme::Dark
    }
}
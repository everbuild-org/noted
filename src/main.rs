mod message;
mod vault;
mod system_config;
mod theme;

use std::sync::Mutex;
use iced::{Application, Command, Element, executor, Renderer, Settings, Theme, window};
use iced::widget::text;
use iced::window::icon;
use crate::message::RootMessage;
use crate::system_config::init_system;
use crate::vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct NotedFlags(Vault);

impl Default for NotedFlags {
    fn default() -> Self {
        NotedFlags(init_system())
    }
}

#[derive(Debug)]
struct Noted {
    vault: Mutex<Vault>
}

impl Application for Noted {
    type Executor = executor::Default;
    type Message = RootMessage;
    type Theme = Theme;
    type Flags = NotedFlags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Noted {
                vault: Mutex::new(flags.0)
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        format!("Noted!{}", VERSION)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        text("Hello, world!").into()
    }

    fn theme(&self) -> Self::Theme {
        self.vault.lock().unwrap().vault_config.lock().unwrap().theme.into()
    }
}

fn main() -> iced::Result {
    Noted::run(Settings {
        window: window::Settings {
            icon: Some(icon::from_file_data(include_bytes!("../icons/icon.png"), None).expect("Invalid icon")),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

mod message;
mod vault;
mod system_config;
mod theme;

use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Mutex;
use std::io::Write;
use std::rc::Rc;
use env_logger::Builder;
use gpui::{App, AppContext, Bounds, div, Hsla, IntoElement, ParentElement, Point, px, relative, Render, Rgba, Styled, TitlebarOptions, ViewContext, VisualContext, WindowBounds, WindowKind, WindowOptions};
use gpui::Fill::Color;
use lazy_static::lazy_static;
use log::info;
use crate::system_config::init_system;
use crate::theme::Theme;
use crate::vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MONTSERRAT: &[u8] = include_bytes!("../data/montserrat/fonts/ttf/Montserrat-Medium.ttf");

#[derive(Debug)]
struct Noted {
    vault: Rc<Mutex<Vault>>,
    theme: RefCell<Theme>,
}

impl Render for Noted {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w(relative(1.0))
            .h(relative(1.0))
            .bg(self.theme.borrow().background.fill())
            .text_color(&self.theme.borrow().foreground)
            .font("Montserrat")
            .child("Hello, world!")
    }
}

fn main() {
    init_logger();

    info!("Starting up Noted v{}", VERSION);
    App::new().run(app);
}

fn app(cx: &mut AppContext) {
    let vault = Rc::new(Mutex::new(init_system()));

    cx.text_system().add_fonts(vec![Cow::Borrowed(&MONTSERRAT)]).unwrap();

    cx.open_window(WindowOptions {
        bounds: WindowBounds::Fixed(Bounds::from_corners(Point::new(0f64.into(), 0f64.into()), Point::new(800f64.into(), 600f64.into()))),
        titlebar: Some(TitlebarOptions {
            title: Some(format!("Noted! {}", VERSION).into()),
            ..Default::default()
        }),
        ..Default::default()
    }, |cx| {
        let system_theme: Theme = vault.lock().unwrap().vault_config.try_lock().unwrap().theme.clone().into();
        let noted = Noted {
            vault,
            theme: RefCell::new(system_theme),
        };

        cx.new_view(|_cx| noted)
    });
}

fn init_logger() {
    Builder::new()
        .parse_default_env()
        .format(|buf, record| {
            use env_logger::fmt::Color;

            let subtle = buf
                .style()
                .set_color(Color::Black)
                .set_intense(true)
                .clone();
            write!(buf, "{}", subtle.value("["))?;
            write!(
                buf,
                "{} ",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%:z")
            )?;
            write!(buf, "{:<5}", buf.default_styled_level(record.level()))?;
            if let Some(path) = record.module_path() {
                write!(buf, " {}", path)?;
            }
            write!(buf, "{}", subtle.value("]"))?;
            writeln!(buf, " {}", record.args())
        })
        .filter_level(log::LevelFilter::Warn)
        .filter_module("noted", log::LevelFilter::Trace)
        .init();
}

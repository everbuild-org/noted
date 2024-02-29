mod vault;
mod system_config;
mod theme;
mod ui;
mod icons;

use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Mutex;
use std::io::Write;
use std::rc::Rc;
use env_logger::Builder;
use gpui::{App, AppContext, Bounds, Context, div, Font, Hsla, IntoElement, Model, ParentElement, Point, px, relative, Render, Rgba, Styled, TitlebarOptions, ViewContext, VisualContext, WindowBounds, WindowKind, WindowOptions};
use gpui::Fill::Color;
use lazy_static::lazy_static;
use log::info;
use crate::system_config::init_system;
use crate::theme::Theme;
use crate::ui::Shell;
use crate::vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MONTSERRAT: &[u8] = include_bytes!("../../data/montserrat/fonts/ttf/Montserrat-Medium.ttf");
const LUCIDE: &[u8] = include_bytes!("../../data/lucide/lucide.ttf");

struct BaseModel {
    vault: Rc<RefCell<Vault>>,
    theme: RefCell<Theme>,
}

#[derive(Debug)]
struct Noted {
    model: Model<BaseModel>
}

impl Render for Noted {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let shell = cx.new_view(|_cx| Shell { model: self.model.clone() });

        let theme = self.model.read(&cx).theme.borrow();

        div()
            .w(relative(1.0))
            .h(relative(1.0))
            .bg(theme.background.fill())
            .text_color(&theme.foreground)
            .font("Montserrat")
            .child(shell)
    }
}

fn main() {
    init_logger();

    info!("Starting up Noted v{}", VERSION);
    App::new().run(app);
}

fn app(cx: &mut AppContext) {
    let vault = Rc::new(RefCell::new(init_system()));
    let system_theme: Theme = vault.borrow().vault_config.lock().unwrap().theme.clone().into();

    let base = cx.new_model(|_cx| BaseModel {
        vault: vault.clone(),
        theme: RefCell::new(system_theme.clone()),
    });

    cx.text_system().add_fonts(vec![Cow::Borrowed(&LUCIDE)]).unwrap();
    cx.text_system().add_fonts(vec![Cow::Borrowed(&MONTSERRAT)]).unwrap();

    cx.open_window(WindowOptions {
        bounds: WindowBounds::Fixed(Bounds::from_corners(Point::new(0f64.into(), 0f64.into()), Point::new(800f64.into(), 600f64.into()))),
        titlebar: Some(TitlebarOptions {
            title: Some(format!("Noted! {}", VERSION).into()),
            ..Default::default()
        }),
        ..Default::default()
    }, |cx| {
        let noted = Noted {
            model: base
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
        .filter_level(log::LevelFilter::Info)
        .filter_module("noted", log::LevelFilter::Trace)
        .init();
}

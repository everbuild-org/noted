mod vault;
mod system_config;
mod theme;
mod ui;
mod icon;
mod asset;
mod pane;

use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use env_logger::Builder;
use gpui::{div, relative, App, AppContext, Bounds, Context, IntoElement, Model, ParentElement, Point, Render, Styled, TitlebarOptions, View, ViewContext, VisualContext, WindowBounds, WindowContext, WindowOptions};
use log::info;
use crate::asset::NotedAssetProvider;
use crate::system_config::init_system;
use crate::theme::Theme;
use crate::ui::Shell;
use crate::vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MONTSERRAT: &[u8] = include_bytes!("../../data/montserrat/fonts/ttf/Montserrat-Medium.ttf");

#[derive(Debug, Clone)]
struct VaultReference {
    vault: Rc<RefCell<Vault>>
}

#[derive(Debug)]
struct Noted {
    model: Model<VaultReference>,
    shell: View<Shell>
}

impl Noted {
    pub fn build(vault: Model<VaultReference>, cx: &mut WindowContext) -> View<Self> {
        let view = cx.new_view(|cx| {
            let shell = Shell::build(cx, vault.read(cx).clone());

            Self {
                model: vault,
                shell
            }
        });

        view
    }
}

impl Render for Noted {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let shell = self.shell.clone();

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
    App::new()
        .with_assets(NotedAssetProvider)
        .run(app);
}

fn app(cx: &mut AppContext) {
    let vault = Rc::new(RefCell::new(init_system()));
    let system_theme: Theme = vault.borrow().vault_config.lock().unwrap().theme.clone().into();
    let base = cx.new_model(|_cx| VaultReference { vault });

    cx.text_system().add_fonts(vec![Cow::Borrowed(&MONTSERRAT)]).unwrap();

    cx.open_window(WindowOptions {
        bounds: WindowBounds::Fixed(Bounds::from_corners(Point::new(0f64.into(), 0f64.into()), Point::new(800f64.into(), 600f64.into()))),
        titlebar: Some(TitlebarOptions {
            title: Some(format!("Noted! {}", VERSION).into()),
            ..Default::default()
        }),
        ..Default::default()
    }, |cx| {
        cx.set_global(system_theme);
        Noted::build(base, cx)
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
        .filter_level(log::LevelFilter::Debug)
        .filter_module("gpui::platform::windows", log::LevelFilter::Info)
        .filter_module("noted", log::LevelFilter::Trace)
        .init();
}

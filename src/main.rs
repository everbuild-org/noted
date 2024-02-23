mod message;
mod vault;
mod system_config;
mod theme;

use std::sync::Mutex;
use std::io::Write;
use std::rc::Rc;
use env_logger::Builder;
use gpui::{App, AppContext, IntoElement, Render, ViewContext, VisualContext, WindowOptions};
use log::info;
use crate::system_config::init_system;
use crate::vault::Vault;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Noted {
    vault: Rc<Mutex<Vault>>
}

impl Render for Noted {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        todo!()
    }
}

fn main() {
    init_logger();

    info!("Starting up Noted v{}", VERSION);
    App::new().run(app);
}

fn app(cx: &mut AppContext) {
    let vault = Rc::new(Mutex::new(init_system()));

    cx.open_window(WindowOptions {
        ..Default::default()
    }, |cx| {
        let noted = Noted {
            vault
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

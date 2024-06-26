pub mod asset;
pub mod icon;
pub mod keymap;
pub mod markdown;
mod optional;
pub mod pane;
pub mod prelude;
pub mod system_config;
pub mod theme;
pub mod ui;
pub mod vault;

use crate::asset::NotedAssetProvider;
use crate::optional::initialize_optional;
use crate::system_config::init_system;
use crate::theme::Theme;
use crate::ui::Shell;
use crate::vault::Vault;
use env_logger::Builder;
use gpui::{
    div, font, relative, App, AppContext, Bounds, Context, IntoElement, Model, ParentElement,
    Point, Render, Styled, TitlebarOptions, View, ViewContext, VisualContext, WindowBounds,
    WindowContext, WindowOptions,
};
use log::info;
use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write;
use std::rc::Rc;
use crate::keymap::register_default_keymap;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MONTSERRAT: &[u8] = include_bytes!("../../data/montserrat/fonts/ttf/Montserrat-Medium.ttf");
const MONTSERRAT_ITALIC: &[u8] =
    include_bytes!("../../data/montserrat/fonts/ttf/Montserrat-MediumItalic.ttf");
const MONTSERRAT_BOLD: &[u8] =
    include_bytes!("../../data/montserrat/fonts/ttf/Montserrat-Bold.ttf");

#[derive(Debug, Clone)]
pub struct VaultReference {
    vault: Rc<RefCell<Vault>>,
}

#[derive(Debug)]
pub struct Noted {
    model: Model<VaultReference>,
    shell: View<Shell>,
}

impl Noted {
    pub fn build(vault: Model<VaultReference>, cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let shell = Shell::build(cx, vault.read(cx).clone());

            Self {
                model: vault,
                shell,
            }
        })
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
            .font(font("Montserrat"))
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
    let system_theme: Theme = vault
        .borrow()
        .vault_config
        .lock()
        .unwrap()
        .theme
        .clone()
        .into();
    let base = cx.new_model(|_cx| VaultReference { vault });

    register_default_keymap(cx);

    cx.text_system()
        .add_fonts(vec![
            Cow::Borrowed(&MONTSERRAT),
            Cow::Borrowed(&MONTSERRAT_ITALIC),
            Cow::Borrowed(&MONTSERRAT_BOLD),
        ])
        .unwrap();

    let window = cx.open_window(
        WindowOptions {
            window_bounds: WindowBounds::Windowed(Bounds::from_corners(
                Point::new(0u32.into(), 0u32.into()),
                Point::new(800u32.into(), 600u32.into()),
            ))
                .into(),
            titlebar: Some(TitlebarOptions {
                title: Some(format!("Noted! {}", VERSION).into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        |cx| {
            cx.on_window_should_close(|cx| {
                cx.quit();
                true
            });

            cx.set_global(system_theme);
            initialize_optional(cx);

            Noted::build(base, cx)
        },
    );

    // Focus editor per default
    window.update(cx, |view, cx| {
        cx.update_view(
            &view.shell,
            |shell, cx|
                cx.update_view(
                    &shell.editor,
                    |editor, cx|
                        editor.focus(cx),
                ),
        );

        cx.activate(true);
    }).unwrap();
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
        .filter_module("naga", log::LevelFilter::Error)
        .filter_module("noted", log::LevelFilter::Trace)
        .init();
}

use gpui::{impl_actions, AppContext, KeyBinding};

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, serde::Deserialize, Debug)]
pub struct CursorMove {
    pub direction: MovementDirection,
    pub select: bool,
}

impl_actions!(text, [CursorMove]);

fn cursor_bind(bind: &str, direction: MovementDirection, select: bool) -> KeyBinding {
    KeyBinding::new(bind, CursorMove {
        direction,
        select,
    }, None)
}

pub fn register_default_keymap(cx: &mut AppContext) {
    cx.bind_keys([
        cursor_bind("left", MovementDirection::Left, false),
        cursor_bind("right", MovementDirection::Right, false),
        cursor_bind("up", MovementDirection::Up, false),
        cursor_bind("down", MovementDirection::Down, false),

        cursor_bind("shift-left", MovementDirection::Left, true),
        cursor_bind("shift-right", MovementDirection::Right, true),
        cursor_bind("shift-up", MovementDirection::Up, true),
        cursor_bind("shift-down", MovementDirection::Down, true),
    ]);
}
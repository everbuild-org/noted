use gpui::AppContext;

#[cfg(feature = "discord")]
pub mod discord;


pub fn initialize_optional(cx: &AppContext) {
    #[cfg(feature = "discord")]
    discord::initialize(cx);
}
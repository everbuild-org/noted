use std::thread;
use discord_presence::{Client, Event};
use gpui::AppContext;
use log::info;

pub fn initialize(_cx: &AppContext) {
    thread::spawn(move || {
        let mut drpc = Client::new(1241546720874205244);
        drpc.start();
        drpc.block_until_event(Event::Ready).unwrap();

        drpc.set_activity(|activity| {
            activity.details("Editing a note")
                .assets(|assets| {
                    assets
                        .large_image("notedicon")
                        .large_text("Noted")

                        .small_image("fileformatmarkdown")
                        .small_text("Markdown")
                })
                .timestamps(|timestamps| {
                    timestamps.start(chrono::Utc::now().timestamp() as u64)
                })
        }).unwrap();

        drpc.block_on().unwrap();
    });

    info!("Discord Rich Presence initialized");
}
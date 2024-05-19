use gpui::{Context, div, IntoElement, ParentElement, RenderOnce, View, WindowContext};
use crate::ui::editor::Editor;

#[derive(Debug, Clone, IntoElement)]
pub struct InformationBar(pub View<Editor>);

struct InformationBarData {
    character_count: usize,
    word_count: usize,
}

impl RenderOnce for InformationBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let data = cx.read_model(&self.0.model, |editor, cx| {
            InformationBarData {
                character_count: editor.count_characters(),
                word_count: editor.count_words(),
            }
        });

        div()
            .child(format!("{} characters, {} words", data.character_count, data.word_count))
    }
}
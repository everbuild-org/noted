use gpui::{IntoElement, RenderOnce, WindowContext};
use lucide_gpui::Icon;

#[derive(Clone, IntoElement)]
struct GpuIcon(pub Icon);

impl RenderOnce for GpuIcon {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.0.render(cx)
    }
}
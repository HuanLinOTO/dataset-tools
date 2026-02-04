use gpui::*;

struct LyricFAApp {
    status: String,
}

impl LyricFAApp {
    fn new() -> Self {
        Self {
            status: "LyricFA - Lyric Forced Alignment Tool".to_string(),
        }
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| LyricFAApp::new())
        })
        .unwrap();
    });
}

impl Render for LyricFAApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x2e2e2e))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child("LyricFA")
                    )
                    .child(
                        div()
                            .text_color(rgb(0xcccccc))
                            .child("Automatic lyric forced alignment using ASR models")
                    )
                    .child(
                        div()
                            .p_4()
                            .text_color(rgb(0xffffff))
                            .bg(rgb(0x404040))
                            .child(&self.status)
                    )
            )
    }
}

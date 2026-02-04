use gpui::*;

struct SlurCutterApp {
    status: String,
}

impl SlurCutterApp {
    fn new() -> Self {
        Self {
            status: "SlurCutter - Slur Detection and Cutting Tool".to_string(),
        }
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| SlurCutterApp::new())
        })
        .unwrap();
    });
}

impl Render for SlurCutterApp {
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
                            .child("SlurCutter")
                    )
                    .child(
                        div()
                            .text_color(rgb(0xcccccc))
                            .child("Detect and cut audio slurs for dataset preparation")
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

#![allow(dead_code, unused)]

use bento::*;

struct MyApp {
    sidebar_visible: bool,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            sidebar_visible: true,
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.keyboard.is_just_pressed(Key::L) {
            self.sidebar_visible = !self.sidebar_visible;
        }

        column(vec![
            if self.sidebar_visible {
                button("Hello")
            } else {
                empty()
            }
        ])
        .gap(8.0)
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento Dashboard"));
}

#![allow(dead_code, unused)]

use bento::*;

struct MyApp {
    count: i32,
    text: String,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
            text: "".to_string(),
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        column(vec![
            row(vec![
                text("Hello", Color::WHITE),
                text("World", Color::rgb(1.0, 0.0, 0.0)),
            ]),
            rect(100.0, 100.0, Color::rgb(0.0, 0.0, 1.0)),
            row(vec![
                text("Second row", Color::rgb(0.0, 1.0, 0.0)),
                text("Also second row", Color::WHITE),
            ]),
        ])
    }

    fn fonts(&self, fonts: &mut Fonts) {
        let font = fonts.add("default", "JetBrainsMono Nerd Font Mono", 14.0);
        fonts.set_default(font);
    }
}

fn main() {
    MyApp::run(Settings::default());
}

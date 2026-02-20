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
        if events.keyboard.is_just_pressed(Key::Escape) {
            exit();
        }

        for k in &events.keyboard.just_pressed {
            self.text.push_str(&k.to_string());
        }

        if events.keyboard.is_just_pressed(Key::Backspace) {
            self.text.pop();
        }

        row(vec![text(
            &format!("Text: {}", self.text),
            Color::rgb(1.0, 1.0, 1.0),
        )])
    }

    fn fonts(&self, fonts: &mut Fonts) {
        let font = fonts.add("default", "JetBrainsMono Nerd Font Mono", 14.0);
        fonts.set_default(font);
    }
}

fn main() {
    MyApp::run(Settings::default());
}

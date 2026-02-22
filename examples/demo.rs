#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        row(vec![
            text("12345678901234567890", Color::WHITE).font("code"),
        ]).width(Val::Percent(100.0)).overflow_hidden()
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("code", "JetBrainsMono Nerd Font Mono", 20.0);
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

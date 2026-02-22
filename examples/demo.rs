#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        column(vec![
            text("Normal (400)", Color::WHITE),
            text("Bold (700)", Color::WHITE).font_weight(700),
            text("Black (900)", Color::WHITE).font_weight(900),
            text("Italic", Color::WHITE).italic(),
            text("Bold Italic", Color::WHITE).font_weight(700).italic(),
            text("Big Bold", Color::WHITE).font_size(32.0).font_weight(700),
            text("Custom Font", Color::WHITE).font("heading"),
        ])
        .gap(8.0)
        .padding(Edges::all(24.0))
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("heading", "JetBrainsMono Nerd Font Mono", 40.0);
        fonts.add("body", "Arial", 14.0).default();
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

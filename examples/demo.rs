#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        column(vec![
            text("Small (10px)", Color::WHITE).font_size(10.0),
            text("Default", Color::WHITE),
            text("Large (24px)", Color::WHITE).font_size(24.0),
            text("Huge (48px)", Color::WHITE).font_size(48.0),
        ])
        .gap(8.0)
        .padding(Edges::all(24.0))
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

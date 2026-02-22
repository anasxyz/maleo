#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        row(vec![
            rect(Color::rgb(0.9, 0.9, 0.95))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 4.0, 4.0, 12.0),
            rect(Color::rgb(0.9, 0.9, 0.95))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .border_radius(12.0)
                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 4.0, 8.0, 20.0),
            rect(Color::rgb(0.9, 0.9, 0.95))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .border_radius(60.0)
                .shadow(Color::rgba(0.2, 0.4, 1.0, 0.6), 0.0, 0.0, 24.0),
        ])
        .gap(32.0)
        .padding(Edges::all(48.0))
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

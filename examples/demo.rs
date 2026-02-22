#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        row(vec![
            rect(Color::rgb(0.2, 0.2, 0.3))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .border(Color::WHITE, 2.0),
            rect(Color::rgb(0.2, 0.2, 0.3))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .border_radius(12.0)
                .border(Color::rgb(0.5, 0.8, 1.0), 2.0),
            rect(Color::rgb(0.2, 0.2, 0.3))
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .border_radius(60.0)
                .border(Color::rgb(1.0, 0.5, 0.2), 3.0),
        ])
        .gap(16.0)
        .padding(Edges::all(16.0))
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

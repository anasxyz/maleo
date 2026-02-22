#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        row(vec![
            rect(Color::RED)
                .width(Val::Px(120.0))
                .height(Val::Px(120.0)),
            rect(Color::RED)
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .opacity(0.5),
            rect(Color::RED)
                .width(Val::Px(120.0))
                .height(Val::Px(120.0))
                .opacity(0.2),
        ])
        .gap(16.0)
        .padding(Edges::all(16.0))
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

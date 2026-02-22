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
                .width(Val::Percent(100.0))
                .height(Val::Px(100.0))
                .border_radius(8.0),
            rect(Color::GREEN)
                .width(Val::Px(100.0))
                .height(Val::Px(100.0)),
            rect(Color::BLUE)
                .width(Val::Px(100.0))
                .height(Val::Px(100.0)),
        ])
        .width(Val::Percent(100.0))
        .gap(16.0)
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

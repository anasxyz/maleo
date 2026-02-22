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
                .width(Val::Percent(98.0))
                .height(Val::Px(100.0))
                .shrink(0.0),
            rect(Color::GREEN)
                .width(Val::Px(100.0))
                .height(Val::Px(100.0))
                .shrink(0.0),
            rect(Color::BLUE)
                .width(Val::Px(100.0))
                .height(Val::Px(100.0))
                .shrink(0.0),
        ])
        .width(Val::Percent(100.0))
        .overflow_hidden()
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento UI"));
}

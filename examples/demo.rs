#![allow(dead_code, unused)]

use bento::*;

struct MyApp {}

impl App for MyApp {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self, events: &Events) -> Element {
        row(vec![
            rect(Color::RED).width(Val::Px(100.0)).height(Val::Px(100.0)),
        ])
    }

    fn fonts(&self, fonts: &mut Fonts) {
    }
}

fn main() {
    MyApp::run(Settings::default());
}

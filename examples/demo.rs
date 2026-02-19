#![allow(dead_code, unused)]

use bento::*;
use winit::keyboard::KeyCode;

struct MyApp {
    count: i32,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.keyboard.is_just_pressed(Key::Escape) {
            std::process::exit(0);
        }

        row(vec![
            text(&format!("Pressed: {:?}", events.keyboard.pressed), Color::rgb(1.0, 1.0, 1.0)),
        ])
    }
}

fn main() {
    MyApp::run(Settings::default());
}

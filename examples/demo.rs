#![allow(dead_code, unused)]

use bento::*;

struct MyApp {
    count: i32,
    text: String,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
            text: "".to_string(),
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        column(vec![
            row(vec![
                rect(0.0, 0.0, Color::rgb(1.0, 0.0, 0.0))
                    .width(Size::Percent(25.0))
                    .height(Size::Fill),
                rect(0.0, 0.0, Color::rgb(0.0, 1.0, 0.0))
                    .width(Size::Percent(50.0))
                    .height(Size::Fill),
                rect(0.0, 0.0, Color::rgb(0.0, 0.0, 1.0))
                    .width(Size::Percent(25.0))
                    .height(Size::Fill),
            ])
            .width(Size::Fill)
            .height(Size::Fill),
            row(vec![
                text("Hello", Color::WHITE),
                text("World", Color::rgb(1.0, 0.0, 0.0)),
            ]),
        ])
        .width(Size::Fill)
        .height(Size::Fill)
    }

    fn fonts(&self, fonts: &mut Fonts) {
        let font = fonts.add("default", "JetBrainsMono Nerd Font Mono", 14.0);
        fonts.set_default(font);
    }
}

fn main() {
    MyApp::run(Settings::default());
}

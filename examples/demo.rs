#![allow(dead_code, unused)]

use bento::*;

const BG: Color = Color::new(0.06, 0.06, 0.06, 1.0);
const RED: Color = Color::new(0.85, 0.15, 0.15, 1.0);
const TEXT: Color = Color::new(0.92, 0.92, 0.92, 1.0);
const DIM: Color = Color::new(0.35, 0.35, 0.35, 1.0);

#[derive(Clone)]
enum Action {
    Increment,
    Decrement,
    Reset,
}

struct MyApp {
    count: i32,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn view(&self) -> Element<Action> {
        let color = if self.count > 0 {
            RED
        } else if self.count < 0 {
            DIM
        } else {
            TEXT
        };

        column(vec![
            text(&self.count.to_string(), color)
                .font_size(96.0)
                .font_weight(700)
                .text_align(TextAlign::Center)
                .width(Val::Percent(100.0)),
            text("COUNTER", DIM)
                .font_size(11.0)
                .font_weight(700)
                .text_align(TextAlign::Center)
                .width(Val::Percent(100.0))
                .margin(Margin::bottom(48.0)),
            row(vec![
                button("−").on_click(Action::Decrement),
                button("RESET")
                    .on_click(Action::Reset)
                    .margin(Margin::horizontal(8.0)),
                button("+").on_click(Action::Increment),
            ])
            .align_x(Align::Center)
            .width(Val::Percent(100.0)),
        ])
        .align_x(Align::Center)
        .align_y(Align::Center)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::KeyPressed(Key::Up) => Some(Action::Increment),
            Event::KeyPressed(Key::Down) => Some(Action::Decrement),
            Event::KeyPressed(Key::R) => Some(Action::Reset),
            _ => None,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
            Action::Reset => self.count = 0,
        }
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("ui", "Arial", 14.0).default();
    }
}

fn main() {
    MyApp::run(
        Settings::default()
            .title("Counter")
            .width(400)
            .height(400)
            .clear_color(BG),
    );
}

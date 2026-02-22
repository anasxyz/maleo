#![allow(dead_code, unused)]

use bento::*;

const BG: Color = Color::new(0.06, 0.06, 0.06, 1.0);
const RED: Color = Color::new(0.85, 0.15, 0.15, 1.0);
const TEXT: Color = Color::new(0.92, 0.92, 0.92, 1.0);
const DIM: Color = Color::new(0.35, 0.35, 0.35, 1.0);

#[derive(Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

struct MyApp {
    count: i32,
}

impl App for MyApp {
    type Message = Message;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn view(&self, events: &Events) -> Element<Message> {
        let count_color = if self.count > 0 {
            RED
        } else if self.count < 0 {
            DIM
        } else {
            TEXT
        };

        column(vec![
            text(&self.count.to_string(), count_color)
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
                button("−").on_click(Message::Decrement),
                button("RESET")
                    .on_click(Message::Reset)
                    .margin(Margin::horizontal(8.0)),
                button("+").on_click(Message::Increment),
            ])
            .align_x(Align::Center)
            .width(Val::Percent(100.0)),
            text("↑ ↓ to count  •  R to reset", DIM)
                .font_size(11.0)
                .text_align(TextAlign::Center)
                .width(Val::Percent(100.0))
                .margin(Margin::top(24.0)),
        ])
        .align_x(Align::Center)
        .align_y(Align::Center)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
        }
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("ui", "Arial", 14.0).default();
    }
}

fn main() {
    MyApp::run(
        Settings::default()
            .title("demo")
            .width(400)
            .height(400)
            .clear_color(BG),
    );
}

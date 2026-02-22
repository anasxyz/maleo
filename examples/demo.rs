#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    NameChanged(String),
    Clear,
}

struct MyApp {
    name: String,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    fn view(&self) -> Element<Action> {
        let greeting = if self.name.is_empty() {
            "Type your name above".to_string()
        } else {
            format!("Hello, {}!", self.name)
        };

        column(vec![
            text_input("name", "Enter your name...")
                .value(&self.name)
                .on_change(|val| Action::NameChanged(val))
                .width(Val::Px(300.0)),
            row(vec![
                text(&greeting, Color::WHITE),
                button("Clear").on_click(Action::Clear),
            ])
            .gap(12.0),
        ])
        .gap(16.0)
        .align_x(Align::Center)
        .align_y(Align::Center)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::NameChanged(val) => self.name = val,
            Action::Clear => self.name = String::new(),
        }
        vec![]
    }
}

fn main() {
    MyApp::run(Settings::default().title("Text Input Test"));
}

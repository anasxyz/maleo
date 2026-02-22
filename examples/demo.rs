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

        row(vec![
            text_input("name")
                .placeholder("Enter your name...")
                .value(&self.name)
                .on_change(|val| Action::NameChanged(val))
                .width(Val::Px(300.0))
                .background(Color::hex("#0a0a15"))
                .border(Color::hex("#8080cc"), 1.0)
                .border_radius(8.0)
                .padding(Edges::all(12.0))
                .text_color(Color::WHITE)
                .placeholder_color(Color::hex("#ffffff4d"))
                .font("mono")
                .font_size(16.0)
                .opacity(0.9),
        ])
        .gap(16.0)
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

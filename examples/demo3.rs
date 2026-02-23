#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    UpdateName(String),
    Submit,
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
        column(vec![
            text("Hello, Bento!", Color::hex("#ffffff"))
                .font_size(24.0)
                .font_weight(700),
            text_input("name")
                .value(&self.name)
                .placeholder("Enter your name...")
                .on_change(|v| Action::UpdateName(v))
                .width(px(300.0)),
            button("Submit")
                .on_click(Action::Submit)
                .background(Color::hex("#2563eb"))
                .border_radius(6.0),
        ])
        .padding(Edges::all(32.0))
        .gap(12.0)
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::UpdateName(v) => self.name = v,
            Action::Submit => {
                println!("Hello, {}!", self.name);
            }
        }
        vec![]
    }
}

fn main() {
    MyApp::run(Settings::default());
}

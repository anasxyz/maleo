#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone, Copy)]
enum Action {
    Increment,
    Decrement,
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
        row(vec![
            button("+").on_click(Action::Increment),
            text(&format!("count: {}", self.count), Color::WHITE).font_size(24.0),
            button("-").on_click(Action::Decrement),
        ])
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        vec![]
    }
}

fn main() {
    MyApp::run(Settings::default());
}


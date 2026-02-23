#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone, Copy)]
enum Action {}

struct MyApp {
    count: i32,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn view(&self) -> Element<Action> {
        column(vec![
            text("Billing address: ", Color::hex("#1f282d"))
                .font_size(12.0)
                .font_weight(500),
            text_input("address")
                .height(Val::Px(100.0))
                .font_size(12.0)
                .border(Color::hex("#000000"), 0.0)
                .margin(Margin::top(3.0))
                .border_radius(10.0)
                .background(Color::hex("#ffffff")),
        ])
        .background(Color::hex("#eeeeee"))
        .padding(Edges {
            top: 5.0,
            left: 5.0,
            right: 0.0,
            bottom: 0.0,
        }) // use padding, not margin
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {}
        vec![]
    }
}

fn main() {
    MyApp::run(Settings::default().clear_color(Color::hex("#eeeeee")));
}

#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    UpdateAddress(String),
}

struct MyApp {
    count: i32,
    address: String,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self { 
            count: 0,
            address: String::new()
        }
    }

    fn view(&self) -> Element<Action> {
        column(vec![
            text("Billing address:", Color::hex("#1f282d"))
                .font_size(12.0)
                .font_weight(500),
            text_input("address")
                .value(&self.address)
                .on_change(|v| Action::UpdateAddress(v))
                .placeholder("Enter address")
                .placeholder_color(Color::hex("#333333"))
                .font_size(12.0)
                .font_weight(500)
                .border(Color::hex("#000000"), 0.0)
                .border_radius(0.0)
                .margin(Margin::top(3.0))
                .background(Color::hex("#1f282d"))
                .text_color(Color::hex("#000000"))
                .width(percent(100.0)),
            {
                if !self.address.is_empty() {
                    text(&self.address, Color::hex("#ffffff"))
                } else {
                    text("No address", Color::hex("#ffffff"))
                }
            }
        ])
        .padding(Edges::all(5.0))
        .width(percent(100.0))
        .height(percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::UpdateAddress(v) => self.address = v,
        }
        vec![]
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("mono", "JetBrainsMono Nerd Font Mono", 24.0);
    }
}

fn main() {
    MyApp::run(Settings::default());
}

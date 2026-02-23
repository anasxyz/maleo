#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    UpdateName(String),
    Save,
    Cancel,
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
            text("Cursor Demo", Color::hex("#ffffff"))
                .font_size(20.0)
                .font_weight(600)
                .margin(Margin::bottom(16.0)),
            text("Hover the text field — cursor becomes a text caret.", Color::hex("#aaaaaa"))
                .font_size(13.0)
                .margin(Margin::bottom(8.0)),
            text_input("name")
                .value(&self.name)
                .on_change(|v| Action::UpdateName(v))
                .placeholder("Type something...")
                .width(px(320.0))
                .padding(Edges::all(8.0))
                .background(Color::hex("#1e2a30"))
                .text_color(Color::hex("#ffffff"))
                .border(Color::hex("#3a4a55"), 1.0)
                .border_radius(6.0)
                .margin(Margin::bottom(20.0)),
            text("Hover the buttons — cursor becomes a pointer.", Color::hex("#aaaaaa"))
                .font_size(13.0)
                .margin(Margin::bottom(8.0)),
            row(vec![
                button("Save")
                    .on_click(Action::Save)
                    .background(Color::hex("#2563eb"))
                    .border_radius(6.0)
                    .margin(Margin::right(8.0)),
                button("Cancel")
                    .on_click(Action::Cancel)
                    .background(Color::hex("#374151"))
                    .border_radius(6.0),
            ])
            .width(percent(100.0))
            .align_x(Align::Center)
        ])
        .padding(Edges::all(32.0))
        .width(percent(100.0))
        .height(percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::UpdateName(v) => self.name = v,
            Action::Save | Action::Cancel => {}
        }
        vec![]
    }
}

fn main() {
    MyApp::run(Settings::default().title("demo").width(500).height(300));
}

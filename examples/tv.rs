#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    UpdateNotes(String),
    UpdateTitle(String),
    Clear,
}

struct MyApp {
    title: String,
    notes: String,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self {
            title: String::new(),
            notes: String::new(),
        }
    }

    fn view(&self) -> Element<Action> {
        let word_count = self.notes
            .split_whitespace()
            .count();

        let line_count = if self.notes.is_empty() {
            0
        } else {
            self.notes.lines().count()
        };

        column(vec![
            // Header
            text("Notes", Color::hex("#ffffff"))
                .font_size(22.0)
                .font_weight(700)
                .margin(Margin::bottom(4.0)),

            text("A simple text editor widget demo.", Color::hex("#888899"))
                .font_size(13.0)
                .margin(Margin::bottom(24.0)),

            // Title field (single-line TextInput for comparison)
            text("Title", Color::hex("#aaaacc"))
                .font_size(12.0)
                .font_weight(600)
                .margin(Margin::bottom(6.0)),

            text_input("title")
                .value(&self.title)
                .on_change(|v| Action::UpdateTitle(v))
                .placeholder("Untitled note...")
                .width(percent(100.0))
                .padding(Edges::all(10.0))
                .background(Color::hex("#1a1a24"))
                .text_color(Color::hex("#eeeeff"))
                .border(Color::hex("#3a3a55"), 1.5)
                .border_radius(6.0)
                .margin(Margin::bottom(16.0)),

            // Body field (multiline TextEditor)
            text("Body", Color::hex("#aaaacc"))
                .font_size(12.0)
                .font_weight(600)
                .margin(Margin::bottom(6.0)),

            text_editor("notes")
                .value(&self.notes)
                .on_change(|v| Action::UpdateNotes(v))
                .placeholder("Start writing...\n\nSupports:\n  • Multiple lines\n  • Up/Down arrows\n  • Click to place cursor\n  • Click and drag to select\n  • Double-click selects word\n  • Triple-click selects line")
                .font("mono")
                .font_size(24.0)
                .width(percent(100.0))
                .height(px(240.0))
                .padding(Edges::all(10.0))
                .background(Color::hex("#1a1a24"))
                .text_color(Color::hex("#eeeeff"))
                .border(Color::hex("#3a3a55"), 1.5)
                .border_radius(6.0)
                .margin(Margin::bottom(12.0)),

            // Stats row
            row(vec![
                text(
                    &format!("{} lines  ·  {} words", line_count, word_count),
                    Color::hex("#666677"),
                )
                .font_size(12.0)
                .grow(1.0),

                button("Clear")
                    .on_click(Action::Clear)
                    .background(Color::hex("#2a1a1a"))
                    .border(Color::hex("#553333"), 1.0)
                    .border_radius(5.0)
                    .text_color(Color::hex("#cc6666")),
            ])
            .width(percent(100.0))
            .align_y(Align::Center)
            .margin(Margin::bottom(20.0)),
        ])
        .padding(Edges::all(32.0))
        .width(percent(100.0))
        .height(percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::UpdateTitle(v) => self.title = v,
            Action::UpdateNotes(v) => self.notes = v,
            Action::Clear => {
                self.title.clear();
                self.notes.clear();
            }
        }
        vec![]
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("mono", "JetBrainsMono Nerd Font Mono", 24.0);
    }
}

fn main() {
    MyApp::run(
        Settings::default()
            .title("Text Editor Demo")
            .width(580)
            .height(560),
    );
}

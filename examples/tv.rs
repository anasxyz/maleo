#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    UpdateNotes(String),
    SelectTab(usize),
    IncreaseFontSize,
    DecreaseFontSize,
}

struct MyApp {
    notes: String,
    selected: usize,
    files: Vec<&'static str>,
    font_size: f32,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self {
            notes: String::new(),
            selected: 0,
            files: vec!["main.rs", "lib.rs", "config.toml", "README.md"],
            font_size: 18.0,
        }
    }

    fn view(&self) -> Element<Action> {
        let bg = Color::hex("#0f111a");
        let sidebar_bg = Color::hex("#0b0d14");
        let active_bg = Color::hex("#1a1d2e");
        let divider = Color::hex("#1a1d2e");
        let text_dim = Color::hex("#3e4260");
        let text_mid = Color::hex("#6b7094");
        let text_bright = Color::hex("#c8cce8");

        // ── sidebar ──────────────────────────────────────────────────────────
        let file_items: Vec<Element<Action>> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let active = i == self.selected;
                row(vec![
                    text(if active { "▸ " } else { "  " }, text_dim).font_size(11.0),
                    text(*name, if active { text_bright } else { text_mid })
                        .font_size(12.0)
                        .grow(1.0),
                ])
                .width(percent(100.0))
                .align_y(Align::Center)
                .padding(Edges {
                    top: 5.0,
                    bottom: 5.0,
                    left: 10.0,
                    right: 8.0,
                })
                .background(if active { active_bg } else { sidebar_bg })
                .on_click(Action::SelectTab(i))
            })
            .collect();

        let sidebar = column(vec![
            text("EXPLORER", text_dim)
                .font_size(10.0)
                .font_weight(600)
                .margin(Margin {
                    top: Some(14.0),
                    bottom: Some(8.0),
                    left: Some(14.0),
                    right: Some(0.0),
                }),
            column(file_items).width(percent(100.0)),
            text(&format!("Font size: {}", self.font_size), Color::WHITE),
        ])
        .width(px(170.0))
        .height(percent(100.0))
        .background(sidebar_bg);

        // ── editor ───────────────────────────────────────────────────────────
        let active_name = self.files[self.selected];

        let tab = row(vec![
            text(active_name, text_mid)
                .font_size(12.0)
                .margin(Margin::left(16.0)),
        ])
        .width(percent(100.0))
        .height(px(30.0))
        .align_y(Align::Center)
        .background(Color::hex("#0d0f1a"));

        let editor = column(vec![
            tab,
            // 1px line under tab
            rect(divider).width(percent(100.0)).height(px(1.0)),
            text_editor("body")
                .value(&self.notes)
                .on_change(|v| Action::UpdateNotes(v))
                .placeholder("// start typing...💁👌🎍😍 السَّلَامُ عَلَيْكُمْ")
                .font("mono")
                .font_size(self.font_size)
                .grow(1.0)
                .width(percent(100.0))
                .padding(Edges {
                    top: 12.0,
                    bottom: 12.0,
                    left: 16.0,
                    right: 16.0,
                })
                .background(bg)
                .text_color(text_bright)
                .border(divider, 0.0),
        ])
        .grow(1.0)
        .height(percent(100.0));

        // ── root ─────────────────────────────────────────────────────────────
        row(vec![
            sidebar,
            rect(divider).width(px(1.0)).height(percent(100.0)),
            editor,
        ])
        .width(percent(100.0))
        .height(percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::UpdateNotes(v) => self.notes = v,
            Action::SelectTab(i) => self.selected = i,
            Action::IncreaseFontSize => {
                self.font_size = (self.font_size + 1.0).min(48.0);
            }
            Action::DecreaseFontSize => {
                self.font_size = (self.font_size - 1.0).max(8.0);
            }
        }
        vec![]
    }

    fn event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::KeyPressed { key: Key::Up, ctrl: true, .. } => Some(Action::IncreaseFontSize),
            Event::KeyPressed { key: Key::Down, .. } => Some(Action::DecreaseFontSize),
            _ => None,
        }
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts
            .add("mono", "JetBrainsMono Nerd Font Mono", 13.0)
            .default();
    }
}

fn main() {
    MyApp::run(
        Settings::default()
            .title("demo")
            .width(800)
            .height(550)
            .clear_color(Color::hex("#0f111a")),
    );
}

#![allow(dead_code, unused)]

use bento::*;

#[derive(Clone)]
enum Action {
    // text input
    SearchChanged(String),

    // buttons
    AddItem,
    RemoveItem(usize),

    // hover tracking
    HoverButton(usize),
    UnhoverButton,

    // tab switching
    SelectTab(usize),
}

struct MyApp {
    search: String,
    items: Vec<String>,
    hovered_item: Option<usize>,
    active_tab: usize,
}

impl App for MyApp {
    type Action = Action;

    fn new() -> Self {
        Self {
            search: String::new(),
            items: vec![
                "Buy groceries".to_string(),
                "Walk the dog".to_string(),
                "Read a book".to_string(),
            ],
            hovered_item: None,
            active_tab: 0,
        }
    }

    fn view(&self) -> Element<Action> {
        let tabs = ["Tasks", "About"];

        column(vec![
            // top bar
            row(vec![
                text("Bento Demo", Color::WHITE).font_size(20.0),
                row(tabs
                    .iter()
                    .enumerate()
                    .map(|(i, label)| {
                        let active = self.active_tab == i;
                        button(label)
                            .background(if active {
                                Color::hex("#5566ee")
                            } else {
                                Color::hex("#2a2a35")
                            })
                            .border_radius(6.0)
                            .on_click(Action::SelectTab(i))
                    })
                    .collect::<Vec<_>>())
                .gap(8.0)
                .align_self(Align::Center),
            ])
            .width(Val::Percent(100.0))
            .align_y(Align::Center)
            .padding(Edges::all(20.0))
            .background(Color::hex("#1a1a24"))
            .gap(16.0),
            // content
            if self.active_tab == 0 {
                self.tasks_tab()
            } else {
                self.about_tab()
            },
        ])
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::SearchChanged(val) => self.search = val,
            Action::AddItem => {
                if !self.search.is_empty() {
                    self.items.push(self.search.clone());
                    self.search = String::new();
                }
            }
            Action::RemoveItem(i) => {
                self.items.remove(i);
            }
            Action::HoverButton(i) => self.hovered_item = Some(i),
            Action::UnhoverButton => self.hovered_item = None,
            Action::SelectTab(i) => self.active_tab = i,
        }
        vec![]
    }
}

impl MyApp {
    fn tasks_tab(&self) -> Element<Action> {
        let filtered: Vec<(usize, &String)> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                self.search.is_empty() || item.to_lowercase().contains(&self.search.to_lowercase())
            })
            .collect();

        column(vec![
            // search + add row
            row(vec![
                text_input("search")
                    .placeholder("New task or search...")
                    .value(&self.search)
                    .on_change(|v| Action::SearchChanged(v))
                    .font_size(50.0)
                    .font("mono")
                    .text_color(Color::hex("#333333"))
                    .background(Color::hex("#FFFFFF"))
                    .border(Color::hex("#888899"), 1.0)
                    .border_radius(1.0)
                    .grow(1.0),
                button("Add")
                    .text_color(Color::BLACK)
                    .background(Color::hex("#44bb77"))
                    .border_radius(6.0)
                    .on_click(Action::AddItem)
                    .height(Val::Percent(100.0))
            ])
            .gap(8.0)
            .width(Val::Percent(100.0)),
            // item count
            text(
                &format!(
                    "{} task{}",
                    filtered.len(),
                    if filtered.len() == 1 { "" } else { "s" }
                ),
                Color::hex("#888899"),
            ),
            // list
            column(if filtered.is_empty() {
                vec![text("No tasks found", Color::hex("#555566")).align_self(Align::Center)]
            } else {
                filtered
                    .iter()
                    .map(|(i, item)| {
                        let hovered = self.hovered_item == Some(*i);
                        row(vec![
                            text(item, Color::WHITE).grow(1.0),
                            button("✕")
                                .background(Color::hex("#cc4444"))
                                .border_radius(4.0)
                                .on_click(Action::RemoveItem(*i)),
                        ])
                        .padding(Edges::all(10.0))
                        .gap(8.0)
                        .width(Val::Percent(100.0))
                                .overflow_hidden()
                        .background(if hovered {
                            Color::hex("#2a2a3e")
                        } else {
                            Color::hex("#1e1e2a")
                        })
                        .border_radius(6.0)
                        .on_hover(Action::HoverButton(*i))
                    })
                    .collect::<Vec<_>>()
            })
            .gap(6.0)
            .width(Val::Percent(100.0)),
        ])
        .gap(12.0)
        .padding(Edges::all(20.0))
        .width(Val::Percent(100.0))
        .grow(1.0)
    }

    fn about_tab(&self) -> Element<Action> {
        column(vec![
            text("Bento UI", Color::WHITE).font_size(28.0),
            text(
                "A Rust UI framework built on wgpu + taffy.",
                Color::hex("#aaaacc"),
            ),
            text("Features so far:", Color::hex("#888899")),
            column(vec![
                text("• Flexbox layout via Taffy", Color::hex("#ccccdd")),
                text("• Text rendering via Glyphon", Color::hex("#ccccdd")),
                text(
                    "• Shapes + shadows + rounded corners",
                    Color::hex("#ccccdd"),
                ),
                text("• Controlled text input", Color::hex("#ccccdd")),
                text(
                    "• on_click, on_hover, on_mouse_down on any element",
                    Color::hex("#ccccdd"),
                ),
            ])
            .gap(4.0),
        ])
        .gap(16.0)
        .padding(Edges::all(32.0))
        .align_x(Align::Center)
        .width(Val::Percent(100.0))
        .grow(1.0)
    }

    fn fonts(&self, fonts: &mut Fonts) {
        fonts.add("mono", "JetBrainsMono Nerd Font Mono", 24.0);
    }
}

fn main() {
    MyApp::run(
        Settings::default()
            .title("Bento Demo")
            .width(640)
            .height(500),
    );
}

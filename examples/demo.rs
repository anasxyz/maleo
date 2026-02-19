use bento::*;
use winit::keyboard::KeyCode;

struct MyApp {
    count: i32,
    sidebar_open: bool,
    bg_color: Color,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
            sidebar_open: false,
            bg_color: Color::rgb(0.22, 0.28, 0.38),
        }
    }

    fn clear_color(&self) -> Color {
        Color::rgb(0.13, 0.13, 0.16)
    }

    fn update(&mut self, events: &Events) -> Element<Self> {
        if events.keyboard.is_just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        let count = self.count;
        let sidebar_open = self.sidebar_open;

        row(vec![
            if sidebar_open {
                container(
                    Color::rgb(0.17, 0.17, 0.21),
                    column(vec![
                        text("Sidebar", Color::rgb(0.7, 0.7, 0.8)),
                        button("Action 1", 160.0, 40.0, Color::rgb(0.2, 0.45, 0.55))
                            .on_click(|app: &mut MyApp| app.count += 10),
                        button("Action 2", 160.0, 40.0, Color::rgb(0.2, 0.45, 0.55))
                            .on_click(|app: &mut MyApp| app.count -= 10),
                    ])
                    .gap(12.0),
                )
                .padding(Padding::all(16.0))
                .height(Size::fill())
            } else {
                empty()
            },
            column(vec![
                container(
                    Color::rgb(0.17, 0.17, 0.21),
                    text("Bento UI Demo", Color::rgb(0.85, 0.85, 0.9)),
                )
                .padding(Padding::all(14.0))
                .width(Size::fill()),
                column(vec![
                    container(
                        self.bg_color,
                        column(vec![
                            text("Counter:", Color::rgb(0.75, 0.8, 0.9)),
                            text(&format!("{}", count), Color::rgb(0.4, 0.75, 0.9)),
                        ])
                        .gap(8.0),
                    )
                    .padding(Padding::all(16.0))
                    .width(Size::fill()),
                    row(vec![
                        button("+ Increment", 110.0, 38.0, Color::rgb(0.2, 0.5, 0.45))
                            .on_click(|app: &mut MyApp| app.count += 1),
                        button("- Decrement", 110.0, 38.0, Color::rgb(0.2, 0.5, 0.45))
                            .on_click(|app: &mut MyApp| app.count -= 1),
                        button("Reset", 80.0, 38.0, Color::rgb(0.45, 0.45, 0.5))
                            .on_click(|app: &mut MyApp| app.count = 0),
                    ])
                    .gap(10.0),
                    column(vec![
                        text("Select Background Color:", Color::rgb(0.7, 0.7, 0.75)),
                        row(vec![
                            button("Red", 70.0, 36.0, Color::rgb(0.65, 0.2, 0.2)).on_click(
                                |app: &mut MyApp| app.bg_color = Color::rgb(0.5, 0.15, 0.15),
                            ),
                            button("Green", 80.0, 36.0, Color::rgb(0.2, 0.55, 0.25)).on_click(
                                |app: &mut MyApp| app.bg_color = Color::rgb(0.1, 0.4, 0.15),
                            ),
                            button("Blue", 70.0, 36.0, Color::rgb(0.2, 0.35, 0.65)).on_click(
                                |app: &mut MyApp| app.bg_color = Color::rgb(0.1, 0.2, 0.5),
                            ),
                        ])
                        .gap(8.0),
                    ])
                    .gap(8.0),
                    button(
                        if sidebar_open {
                            "Hide Sidebar"
                        } else {
                            "Toggle Sidebar"
                        },
                        140.0,
                        38.0,
                        Color::rgb(0.2, 0.5, 0.45),
                    )
                    .on_click(|app: &mut MyApp| app.sidebar_open = !app.sidebar_open),
                    text("Press ESC to exit", Color::rgb(0.4, 0.4, 0.45)),
                ])
                .gap(16.0)
                .padding(Padding::all(20.0))
                .width(Size::fill()),
            ])
            .width(Size::fill()),
        ])
    }
}

fn main() {
    bento::run::<MyApp>("Bento", 800, 520);
}

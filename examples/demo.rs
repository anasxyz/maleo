use bento::*;

struct MyApp {
    count: i32,
    sidebar_open: bool,
}

impl App for MyApp {
    fn new() -> Self {
        Self { count: 0, sidebar_open: false }
    }

    fn clear_color() -> Color {
        Color::rgb(0.11, 0.11, 0.13)
    }

    fn update(&mut self, _events: &Events) -> Element<Self> {
        let count = self.count;
        let sidebar_open = self.sidebar_open;

        row(vec![
            if sidebar_open {
                column(vec![
                    button("Reset",   180.0, 44.0, Color::rgb(0.55, 0.15, 0.15)).on_click(|app: &mut MyApp| app.count = 0),
                    button("Set 10",  180.0, 44.0, Color::rgb(0.15, 0.35, 0.55)).on_click(|app: &mut MyApp| app.count = 10),
                    button("Set 100", 180.0, 44.0, Color::rgb(0.15, 0.35, 0.55)).on_click(|app: &mut MyApp| app.count = 100),
                    button("Negate",  180.0, 44.0, Color::rgb(0.3, 0.15, 0.55)).on_click(|app: &mut MyApp| app.count = -app.count),
                ]).padding(Padding::right(12.0))
            } else {
                empty()
            },

            column(vec![
                button(
                    if sidebar_open { "✕ Close" } else { "⚙ Options" },
                    110.0, 36.0,
                    Color::rgb(0.22, 0.22, 0.26),
                ).on_click(|app: &mut MyApp| app.sidebar_open = !app.sidebar_open),

                text(&format!("{}", count), Color::WHITE),

                row(vec![
                    button("-", 56.0, 56.0, Color::rgb(0.65, 0.2, 0.2)).on_click(|app: &mut MyApp| app.count -= 1),
                    button("+", 56.0, 56.0, Color::rgb(0.2, 0.55, 0.25)).on_click(|app: &mut MyApp| app.count += 1),
                ]),
            ]).padding(Padding::all(20.0)),
        ])
    }
}

fn main() {
    bento::run::<MyApp>("Counter", 500, 320);
}

#![allow(dead_code, unused)]

use bento::*;

struct MyApp {
    sidebar_visible: bool,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            sidebar_visible: true,
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.keyboard.is_just_pressed(Key::L) {
            self.sidebar_visible = !self.sidebar_visible;
        }

        let bg       = Color::rgb(0.08, 0.08, 0.10);
        let surface  = Color::rgb(0.12, 0.12, 0.15);
        let surface2 = Color::rgb(0.16, 0.16, 0.20);
        let accent   = Color::rgb(0.38, 0.65, 1.0);
        let green    = Color::rgb(0.35, 0.85, 0.55);
        let yellow   = Color::rgb(0.95, 0.78, 0.35);
        let red      = Color::rgb(0.95, 0.38, 0.38);
        let text_primary   = Color::rgb(0.92, 0.92, 0.95);
        let text_secondary = Color::rgb(0.50, 0.50, 0.58);

        let sidebar = column(vec![
            text("MAIN", text_secondary),
            text("Dashboard", text_primary),
            text("Projects", text_secondary),
            text("Team", text_secondary),
            text("Reports", text_secondary),
            text("ACCOUNT", text_secondary),
            text("Profile", text_secondary),
            text("Billing", text_secondary),
            text("Logout", red),
        ])
        .width(Size::Fixed(180.0))
        .height(Size::Fill)
        .padding(Padding::all(24.0))
        .gap(14.0)
        .background(surface);

        let main_content = column(vec![
            // stat cards
            row(vec![
                column(vec![
                    text("Total Revenue", text_secondary),
                    text("$48,295", text_primary),
                    text("↑ 12.4% this month", green),
                ])
                .width(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(6.0)
                .background(surface2),

                column(vec![
                    text("Active Users", text_secondary),
                    text("3,842", text_primary),
                    text("↑ 8.1% this month", green),
                ])
                .width(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(6.0)
                .background(surface2),

                column(vec![
                    text("Open Tickets", text_secondary),
                    text("27", text_primary),
                    text("↓ 3 since yesterday", yellow),
                ])
                .width(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(6.0)
                .background(surface2),

                column(vec![
                    text("Error Rate", text_secondary),
                    text("0.42%", text_primary),
                    text("↑ needs attention", red),
                ])
                .width(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(6.0)
                .background(surface2),
            ])
            .width(Size::Fill)
            .gap(12.0),

            // lower panels
            row(vec![
                column(vec![
                    text("Recent Activity", text_primary),
                    text("────────────────────", text_secondary),
                    row(vec![text("●", green),  text("User @alice signed up", text_primary)]).gap(8.0),
                    row(vec![text("●", accent), text("Invoice #1042 paid", text_primary)]).gap(8.0),
                    row(vec![text("●", yellow), text("Deploy to prod started", text_primary)]).gap(8.0),
                    row(vec![text("●", red),    text("API error spike detected", text_primary)]).gap(8.0),
                    row(vec![text("●", green),  text("Backup completed", text_primary)]).gap(8.0),
                ])
                .width(Size::Fill)
                .height(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(12.0)
                .background(surface2),

                column(vec![
                    text("System Health", text_primary),
                    text("────────────────────", text_secondary),
                    row(vec![text("CPU",     text_secondary), text("42%",   green)]).width(Size::Fill).gap(8.0),
                    row(vec![text("Memory",  text_secondary), text("71%",   yellow)]).width(Size::Fill).gap(8.0),
                    row(vec![text("Disk",    text_secondary), text("88%",   red)]).width(Size::Fill).gap(8.0),
                    row(vec![text("Network", text_secondary), text("12ms",  green)]).width(Size::Fill).gap(8.0),
                    row(vec![text("Uptime",  text_secondary), text("99.9%", green)]).width(Size::Fill).gap(8.0),
                ])
                .width(Size::Percent(30.0))
                .height(Size::Fill)
                .padding(Padding::all(20.0))
                .gap(12.0)
                .background(surface2),
            ])
            .width(Size::Fill)
            .height(Size::Fill)
            .gap(12.0),
        ])
        .width(Size::Fill)
        .height(Size::Fill)
        .padding(Padding::all(24.0))
        .gap(12.0);

        let mut body_children = vec![];
        if self.sidebar_visible {
            body_children.push(sidebar);
        }
        body_children.push(main_content);

        column(vec![
            // topbar
            row(vec![
                text("bento", accent),
                row(vec![
                    text("Overview",  text_primary),
                    text("Analytics", text_secondary),
                    text("Settings",  text_secondary),
                ])
                .gap(32.0)
                .width(Size::Fill)
                .align_x(Align::Center),
                text("Press L to toggle sidebar", text_secondary),
            ])
            .width(Size::Fill)
            .padding(Padding::horizontal(32.0))
            .padding(Padding::vertical(16.0))
            .background(surface),

            // body
            row(body_children)
                .width(Size::Fill)
                .height(Size::Fill),
        ])
    }
}

fn main() {
    MyApp::run(Settings::default().title("Bento Dashboard"));
}

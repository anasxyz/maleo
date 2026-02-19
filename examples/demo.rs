use bento::*;
use glyphon::TextArea;
use winit::keyboard::KeyCode;

struct MyApp {
    count: i32,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
        }
    }

    fn clear_color(&self) -> Color {
        Color::rgb(0.13, 0.13, 0.16)
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.keyboard.is_just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

    }
}

fn main() {
    bento::run::<MyApp>("Bento", 800, 520);
}

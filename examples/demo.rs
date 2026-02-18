use bento::*;
use winit::keyboard::KeyCode;

struct MyApp {
    count: u32,
}

impl App for MyApp {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, input: &Input) -> Element {
        if input.left_just_pressed {
            self.count += 1;
        }

        row(
            10.0,
            vec![
                if input.is_key_pressed(KeyCode::KeyA) {
                    text("A", Color::RED)
                } else {
                    text("A", Color::WHITE)
                },
                text(&format!("Count: {}", self.count), Color::WHITE),
                text(&format!("Count: {}", self.count), Color::WHITE),
                text(&format!("Count: {}", self.count), Color::WHITE),
            ],
        )
    }
}

fn main() {
    bento::run::<MyApp>("Bento", 800, 600);
}

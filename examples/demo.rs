use bento::*;
use winit::keyboard::KeyCode;

struct MyApp {
    count: u32,
    switch: bool,
}

impl App for MyApp {
    fn new() -> Self {
        Self { 
            count: 0,
            switch: false,
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.mouse.left_just_pressed {
            self.count += 1;
        }

        if events.keyboard.is_just_pressed(KeyCode::KeyS) {
            self.switch = !self.switch;
        }

        row(
            10.0,
            vec![
                if self.switch {
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

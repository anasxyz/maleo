use bento::*;
use winit::keyboard::KeyCode;

struct MyApp {
    count: u32,
    switch: bool,
    num_texts: u32,
}

impl App for MyApp {
    fn new() -> Self {
        Self {
            count: 0,
            switch: false,
            num_texts: 0,
        }
    }

    fn update(&mut self, events: &Events) -> Element {
        if events.mouse.left_just_pressed {
            self.count += 1;
        }
        if events.keyboard.is_just_pressed(KeyCode::KeyF) {
            self.num_texts += 1;
        }

        let mut children = vec![];
        for _ in 0..self.num_texts {
            children.push(text(&format!("Count: {}", self.count), Color::WHITE));
        }

        row(0.0, children)
    }
}

fn main() {
    bento::run::<MyApp>("Bento", 800, 600);
}

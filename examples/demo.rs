#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use bento::{App, Color, Ctx};

struct MyApp { count: u32 }

impl App for MyApp {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, ctx: &mut Ctx) {
        ctx.draw_text(format!("Hello world! {}", self.count).as_str(), 20.0, 20.0, Color::WHITE);
        println!("count: {}", self.count);
        if ctx.mouse.left_just_pressed { self.count += 1; }
    }
}

fn main() {
    bento::run::<MyApp>("Bento demo", 800, 600);
}

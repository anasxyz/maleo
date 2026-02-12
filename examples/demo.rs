#![allow(dead_code, unused)]
use winit::keyboard::KeyCode;

use bento::{App, BentoApp, Color, Ctx, FontId, Widget, Container, ContainerDirection};

struct Demo {
    x: f32,
}

impl BentoApp for Demo {
    fn once(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("main_font", "JetBrainsMono Nerd Font", 14.0);

        // Create some rects
        let rect1 = Widget::Rect(bento::Rect {
            id: 0,
            x: 0.0,
            y: 0.0,
            w: 80.0,
            h: 40.0,
            color: Color::RED,
            outline_color: Color::BLACK,
            outline_thickness: 0.0,
        });

        let rect2 = Widget::Rect(bento::Rect {
            id: 1,
            x: 0.0,
            y: 0.0,
            w: 80.0,
            h: 40.0,
            color: Color::BLUE,
            outline_color: Color::BLACK,
            outline_thickness: 0.0,
        });

        let hcontainer = Widget::Container(Box::new(Container {
            id: 0,
            direction: ContainerDirection::Horizontal,
            x: 0.0,
            y: 0.0,
            w: 250.0,
            h: 100.0,
            gap: 5.0,
            children: vec![rect1, rect2],
        }));

        let text1 = Widget::Text(bento::Text {
            id: 2,
            text: "Hello World".to_string(),
            font_id: font,
            x: 0.0,
            y: 0.0,
            color: Color::WHITE,
            font_size: 14.0,
            font_family: "JetBrainsMono Nerd Font".to_string(),
        });

        let rect3 = Widget::Rect(bento::Rect {
            id: 3,
            x: 0.0,
            y: 0.0,
            w: 150.0,
            h: 30.0,
            color: Color::GREEN,
            outline_color: Color::BLACK,
            outline_thickness: 0.0,
        });

        // Create outer vertical container with nested container inside
        ctx.hcontainer(10.0, 10.0, 400.0, 300.0, 10.0, vec![hcontainer, text1, rect3]);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.input.keys_just_pressed.contains(&KeyCode::Escape) {
            ctx.exit();
        }
    }
}

fn main() {
    App::new("bento", 440, 260).run(Demo { x: 0.0 });
}

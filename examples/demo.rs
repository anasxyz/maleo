#![allow(dead_code, unused)]
use winit::keyboard::KeyCode;

use bento::{App, BentoApp, Color, Ctx, FontId, Fonts, Height, Width, Rect, Text, Button};

struct Demo {
    x: f32,
    menu_open: bool,
}

impl BentoApp for Demo {
    fn once(&mut self, ctx: &mut Ctx) {
        let font = ctx.ui.fonts.default();

        ctx.ui.rect(
            "rect1",
            200.0,
            0.0,
            Width::Percent(0.8),
            Height::Percent(0.8),
            Color::RED,
            Color::BLACK,
            0.0,
        );

        ctx.ui
            .text("text1", "Hello World", font, 100.0, 10.0, Color::WHITE);

        ctx.ui.button("button1", "Click Me", 10.0, 200.0);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.input.keys_just_pressed.contains(&KeyCode::Escape) {
            ctx.exit();
        }

        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyW) {
            ctx.ui.rects[0].y -= 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyS) {
            ctx.ui.rects[0].y += 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyA) {
            ctx.ui.rects[0].x -= 30.0;
            ctx.mark_dirty();
        }
        if ctx.input.keys_just_pressed.contains(&KeyCode::KeyD) {
            ctx.ui.rects[0].w += 30.0;
            ctx.mark_dirty();
        }

        if ctx.is_clicked("button1") {
            if !self.menu_open {
                ctx.ui.rect(
                    "rect2",
                    0.0,
                    0.0,
                    Width::Fixed(200.0),
                    Height::Full,
                    Color::BLUE,
                    Color::BLACK,
                    0.0,
                );
                self.menu_open = true;
            } else {
                ctx.ui.toggle("rect2");
            }
            ctx.mark_dirty();
        }

        if ctx.ui.exists::<Rect>("rect2") && ctx.ui.get::<Rect>("rect2").unwrap().visible {
            ctx.ui.get_mut::<Rect>("rect1").unwrap().x = 200.0;
        } else {
            ctx.ui.get_mut::<Rect>("rect1").unwrap().x = 0.0;
        }
    }
}

fn main() {
    App::new("bento", 440, 260).run(Demo {
        x: 0.0,
        menu_open: false,
    });
}

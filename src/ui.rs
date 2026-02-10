use crate::{ShapeRenderer, TextRenderer};

pub struct Ui {
    pub text_renderer: TextRenderer,
    pub shape_renderer: ShapeRenderer,
}

impl Ui {
    pub fn new(text_renderer: TextRenderer, shape_renderer: ShapeRenderer) -> Self {
        Self {
            text_renderer,
            shape_renderer,
        }
    }

    pub fn rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .rect(x, y, w, h, color, outline_color, outline_thickness);
    }

    pub fn circle(
        &mut self,
        cx: f32,
        cy: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer
            .circle(cx, cy, radius, color, outline_color, outline_thickness);
    }

    pub fn rounded_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        color: [f32; 4],
        outline_color: [f32; 4],
        outline_thickness: f32,
    ) {
        self.shape_renderer.rounded_rect(
            x,
            y,
            w,
            h,
            radius,
            color,
            outline_color,
            outline_thickness,
        );
    }

    pub fn text(&mut self, text: &str, font_size: f32, x: f32, y: f32) {
        self.text_renderer.draw(text, font_size, x, y);
    }
}

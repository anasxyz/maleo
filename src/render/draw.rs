use crate::render::shape_renderer::{RectParams, ShapeRenderer};
use wgpu;

pub struct DrawContext {
    shapes: ShapeRenderer,
    scale: f32,
}

impl DrawContext {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Self {
        Self {
            shapes: ShapeRenderer::new(device, format, width, height),
            scale,
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.shapes.resize(width as f32, height as f32);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, p: RectParams) {
        let s = self.scale;
        self.shapes.draw_rect(
            x * s,
            y * s,
            w * s,
            h * s,
            RectParams {
                radius: p.radius * s,
                border_width: p.border_width * s,
                ..p
            },
        );
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        self.shapes.render(device, queue, pass);
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }
}

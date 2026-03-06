use crate::render::shape_renderer::{RectParams, ShapeRenderer};
use wgpu;

pub struct DrawContext {
    shapes: ShapeRenderer,
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
            shapes: ShapeRenderer::new(device, format, width, height, scale),
        }
    }

    // logical pixels
    pub fn resize(&mut self, width: f32, height: f32) {
        self.shapes.resize(width, height);
    }

    pub fn set_scale(&mut self, scale: f32, width: f32, height: f32) {
        self.shapes.set_scale(scale, width, height);
    }

    // all inputs in logical pixels
    pub fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, p: RectParams) {
        self.shapes.draw_rect(x, y, w, h, p);
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

use crate::color::Color;
use crate::render::draw::DrawContext;
use crate::render::gpu::GpuContext;
use crate::render::shape_renderer::RectParams;
use std::sync::Arc;
use winit::window::Window;

pub struct WindowState {
    pub window: Arc<Window>,
    pub gpu: GpuContext,
    pub clear_color: Color,
    pub draw: DrawContext,
}

impl WindowState {
    pub fn new(window: Arc<Window>, gpu: GpuContext, clear_color: Color) -> Self {
        let size = window.inner_size();
        let scale = window.scale_factor();
        let draw = DrawContext::new(
            &gpu.device,
            gpu.format,
            size.width as f32 / scale as f32,  // logical
            size.height as f32 / scale as f32, // logical
            scale as f32,
        );
        Self {
            window,
            gpu,
            clear_color,
            draw,
        }
    }

    pub fn begin(&mut self) {
        self.draw.clear();
    }

    pub fn render(&mut self) {
        let frame = match self.gpu.begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view) = frame.begin();

        {
            let c = self.clear_color;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: c.r as f64,
                            g: c.g as f64,
                            b: c.b as f64,
                            a: c.a as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.draw
                .render(&self.gpu.device, &self.gpu.queue, &mut pass);
        }

        finisher.present(encoder, &self.gpu.queue);
    }
}

use std::sync::Arc;
use winit::window::Window;
use crate::color::Color;
use crate::render::gpu::GpuContext;

pub struct WindowState {
    pub window: Arc<Window>,
    pub gpu: GpuContext,
    pub clear_color: Color,
}

impl WindowState {
    pub fn new(window: Arc<Window>, gpu: GpuContext, clear_color: Color) -> Self {
        Self { window, gpu, clear_color }
    }

    pub fn render(&mut self) {
        let frame = match self.gpu.begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view) = frame.begin();

        {
            let c = self.clear_color;
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }

        finisher.present(encoder, &self.gpu.queue);
    }
}

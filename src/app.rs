// src/app.rs - Clean app logic, separated from GPU details

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{GpuContext, ShapeRenderer, TextRenderer, Scene, DrawCommand};

/// The main application
pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    gpu: GpuContext,
    scale_factor: f64,
}

/// Drawing context passed to user code
pub struct Canvas<'a> {
    pub scene: &'a mut Scene,
    pub width: f32,
    pub height: f32,
    pub scale_factor: f64,
}

impl App {
    /// Create a new application
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        pollster::block_on(Self::new_async(title, width, height))
    }

    async fn new_async(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = Arc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(winit::dpi::LogicalSize::new(width, height))
                .build(&event_loop)
                .unwrap(),
        );

        let gpu = GpuContext::new(window.clone()).await;
        let scale_factor = window.scale_factor();

        Self {
            event_loop: Some(event_loop),
            window,
            gpu,
            scale_factor,
        }
    }

    /// Get window size in logical coordinates
    #[inline(always)]
    fn logical_size(&self) -> (f32, f32) {
        (
            (self.gpu.config.width as f64 / self.scale_factor) as f32,
            (self.gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    /// Run the application with a user update function
    pub fn run<F>(mut self, mut update_fn: F)
    where
        F: FnMut(&mut Canvas) + 'static,
    {
        // Create renderers
        let (width, height) = self.logical_size();
        let mut shape_renderer = ShapeRenderer::new(&self.gpu.device, self.gpu.format, width, height);
        let mut text_renderer = TextRenderer::new(&self.gpu.device, &self.gpu.queue, self.gpu.format);
        text_renderer.resize(width, height, self.scale_factor);
        
        let mut scene = Scene::new();
        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            self.on_scale_change(
                                scale_factor,
                                &mut shape_renderer,
                                &mut text_renderer,
                                &mut scene,
                            );
                        }
                        WindowEvent::Resized(new_size) => {
                            self.on_resize(
                                new_size,
                                &mut shape_renderer,
                                &mut text_renderer,
                                &mut scene,
                            );
                        }
                        WindowEvent::RedrawRequested => {
                            self.on_redraw(
                                &mut shape_renderer,
                                &mut text_renderer,
                                &mut scene,
                                &mut update_fn,
                            );
                        }
                        WindowEvent::CloseRequested => {
                            target.exit();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }

    // ========================================================================
    // Event Handlers
    // ========================================================================

    fn on_scale_change(
        &mut self,
        scale_factor: f64,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
    ) {
        self.scale_factor = scale_factor;
        
        let physical_size = self.window.inner_size();
        self.gpu.resize(physical_size.width, physical_size.height);

        let (width, height) = self.logical_size();
        shape_renderer.resize(width, height);
        text_renderer.resize(width, height, self.scale_factor);
        
        scene.mark_dirty();
        self.window.request_redraw();
    }

    fn on_resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
    ) {
        self.gpu.resize(new_size.width, new_size.height);

        let (width, height) = self.logical_size();
        shape_renderer.resize(width, height);
        text_renderer.resize(width, height, self.scale_factor);
        
        scene.mark_dirty();
        self.window.request_redraw();
    }

    fn on_redraw<F>(
        &mut self,
        shape_renderer: &mut ShapeRenderer,
        text_renderer: &mut TextRenderer,
        scene: &mut Scene,
        update_fn: &mut F,
    ) where
        F: FnMut(&mut Canvas),
    {
        // Update scene if dirty
        if scene.is_dirty() {
            scene.clear();
            
            let (width, height) = self.logical_size();
            let mut canvas = Canvas {
                scene,
                width,
                height,
                scale_factor: self.scale_factor,
            };
            
            update_fn(&mut canvas);
        }

        // Render
        let frame = match self.gpu.begin_frame() {
            Ok(frame) => frame,
            Err(_) => return, // Surface lost, skip frame
        };

        // Begin rendering - consumes frame and gives us everything we need
        let (mut encoder, finisher, view, msaa_view) = frame.begin();

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa_view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Process scene commands
            shape_renderer.clear();
            text_renderer.clear();
            
            let (width, height) = self.logical_size();
            
            for cmd in scene.commands() {
                match cmd {
                    DrawCommand::Rect { x, y, w, h, color, outline_color, outline_thickness } => {
                        shape_renderer.rect(*x, *y, *w, *h, *color, *outline_color, *outline_thickness);
                    }
                    DrawCommand::Circle { cx, cy, radius, color, outline_color, outline_thickness } => {
                        shape_renderer.circle(*cx, *cy, *radius, *color, *outline_color, *outline_thickness);
                    }
                    DrawCommand::RoundedRect { x, y, w, h, radius, color, outline_color, outline_thickness } => {
                        shape_renderer.rounded_rect(*x, *y, *w, *h, *radius, *color, *outline_color, *outline_thickness);
                    }
                    DrawCommand::Text { text, x, y } => {
                        text_renderer.queue_text(text, *x, *y, width, height, self.scale_factor);
                    }
                }
            }

            // Render
            shape_renderer.render(&self.gpu.device, &self.gpu.queue, &mut pass);
            text_renderer.render(width, height, self.scale_factor, &self.gpu.device, &self.gpu.queue, &mut pass);
        }

        finisher.present(encoder, &self.gpu.queue);
        scene.mark_clean();
    }
}

use std::sync::Arc;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{GpuContext, MouseState, ShapeRenderer, TextRenderer, Ui};

/// main application
pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    gpu: GpuContext,
    scale_factor: f64,
}

impl App {
    /// create a new application
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

    /// get window size in logical coordinates
    #[inline(always)]
    fn logical_size(&self) -> (f32, f32) {
        (
            (self.gpu.config.width as f64 / self.scale_factor) as f32,
            (self.gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    /// run the application with a user update function
    pub fn run<F>(mut self, mut update_fn: F)
    where
        F: FnMut(&mut Ui) + 'static,
    {
        // create renderers
        let (width, height) = self.logical_size();
        let mut ui = Ui::new(
            TextRenderer::new(&self.gpu.device, &self.gpu.queue, self.gpu.format),
            ShapeRenderer::new(&self.gpu.device, self.gpu.format, width, height),
        );
        ui.text_renderer.resize(width, height, self.scale_factor);

        let mut mouse = MouseState::default();
        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            mouse.x = (position.x / self.scale_factor) as f32;
                            mouse.y = (position.y / self.scale_factor) as f32;
                            // self.window.request_redraw();
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            match button {
                                MouseButton::Left => {
                                    let pressed = state == ElementState::Pressed;
                                    mouse.left_just_pressed = pressed && !mouse.left_pressed;
                                    mouse.left_just_released = !pressed && mouse.left_pressed;
                                    mouse.left_pressed = pressed;
                                }
                                MouseButton::Right => {
                                    mouse.right_pressed = state == ElementState::Pressed;
                                }
                                _ => {}
                            }
                            self.window.request_redraw();
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            self.on_scale_change(&mut ui, scale_factor);
                        }
                        WindowEvent::Resized(new_size) => {
                            self.on_resize(&mut ui, new_size);
                        }
                        WindowEvent::RedrawRequested => {
                            self.on_redraw(&mut ui, &mut mouse, &mut update_fn);
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

    fn on_scale_change(&mut self, ui: &mut Ui, scale_factor: f64) {
        self.scale_factor = scale_factor;

        let physical_size = self.window.inner_size();
        self.gpu.resize(physical_size.width, physical_size.height);

        let (width, height) = self.logical_size();
        ui.shape_renderer.resize(width, height);
        ui.text_renderer.resize(width, height, self.scale_factor);

        self.window.request_redraw();
    }

    fn on_resize(&mut self, ui: &mut Ui, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(new_size.width, new_size.height);

        let (width, height) = self.logical_size();
        ui.shape_renderer.resize(width, height);
        ui.text_renderer.resize(width, height, self.scale_factor);

        self.window.request_redraw();
    }

    fn on_redraw<F>(&mut self, ui: &mut Ui, mouse: &mut MouseState, update_fn: &mut F)
    where
        F: FnMut(&mut Ui),
    {
        println!("redrawing");

        ui.shape_renderer.clear();
        ui.text_renderer.clear();

        let (width, height) = self.logical_size();

        update_fn(ui);

        let frame = match self.gpu.begin_frame() {
            Ok(frame) => frame,
            Err(_) => return,
        };

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

            let (width, height) = self.logical_size();

            ui.shape_renderer
                .render(&self.gpu.device, &self.gpu.queue, &mut pass);
            ui.text_renderer.render(
                width,
                height,
                self.scale_factor,
                &self.gpu.device,
                &self.gpu.queue,
                &mut pass,
            );
        }

        finisher.present(encoder, &self.gpu.queue);
    }
}

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{Fonts, GpuContext, ShapeRenderer, TextRenderer};

pub trait BentoApp: 'static {}

struct WindowState {
    gpu: GpuContext,
    window: Arc<Window>,
    scale_factor: f64,
}

impl WindowState {
    async fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window.clone()).await;
        let scale_factor = window.scale_factor();

        Self {
            window,
            gpu,
            scale_factor,
        }
    }

    fn logical_size(&self) -> (f32, f32) {
        (
            (self.gpu.config.width as f64 / self.scale_factor) as f32,
            (self.gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(new_size.width, new_size.height);
        let (w, h) = self.logical_size();
    }

    fn on_scale_change(
        &mut self,
        scale_factor: f64,
        new_inner_size: winit::dpi::PhysicalSize<u32>,
    ) {
        self.scale_factor = scale_factor;
        self.gpu.resize(new_inner_size.width, new_inner_size.height);
        let (w, h) = self.logical_size();
    }

    fn render<T: BentoApp>(&mut self, app: &mut T) {
        println!("render");

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
            // text_renderer render function goes here
            // shape_renderer render function goes here
        }

        // text_renderer trim_atlas function goes here
        finisher.present(encoder, &self.gpu.queue);
    }
}

struct WinitHandler<T: BentoApp> {
    title: String,
    width: u32,
    height: u32,
    app: T,
    window_state: Option<WindowState>,
    setup_done: bool,
}

impl<T: BentoApp> WinitHandler<T> {
    fn new(title: &str, width: u32, height: u32, app: T) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            app,
            window_state: None,
            setup_done: false,
        }
    }
}

impl<T: BentoApp> ApplicationHandler for WinitHandler<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        let ws = pollster::block_on(WindowState::new(window.clone()));

        let scale_factor = window.scale_factor();
        let physical = window.inner_size();
        let width = (physical.width as f64 / scale_factor) as f32;
        let height = (physical.height as f64 / scale_factor) as f32;

        let mut text_renderer = TextRenderer::new(&ws.gpu.device, &ws.gpu.queue, ws.gpu.format);
        let shape_renderer = ShapeRenderer::new(&ws.gpu.device, ws.gpu.format, width, height);
        text_renderer.resize(width, height, scale_factor);

        self.window_state = Some(ws);

        let mut fonts = Fonts::new();
        fonts.add("default", "Arial", 14.0);

        self.setup_done = true;

        self.window_state.as_ref().unwrap().window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let ws = match self.window_state.as_mut() {
            Some(ws) => ws,
            None => return,
        };

        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let new_x = (position.x / ws.scale_factor) as f32;
                let new_y = (position.y / ws.scale_factor) as f32;

                ws.window.request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {}
                    MouseButton::Right => {}
                    MouseButton::Middle => {}
                    _ => {}
                }

                ws.window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {}
                    MouseScrollDelta::PixelDelta(pos) => {}
                }

                ws.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(key) = event.physical_key {
                    if pressed {
                    } else {
                    }
                }

                ws.window.request_redraw();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => for c in text.chars() {},
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                let new_inner = ws.window.inner_size();
                ws.on_scale_change(scale_factor, new_inner);
                ws.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                ws.on_resize(new_size);
                ws.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                ws.render(&mut self.app);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

pub struct App {
    title: String,
    width: u32,
    height: u32,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub fn run<T: BentoApp>(self, app: T) {
        let event_loop = EventLoop::new().unwrap();
        let mut handler = WinitHandler::new(&self.title, self.width, self.height, app);
        event_loop.run_app(&mut handler).unwrap();
    }
}

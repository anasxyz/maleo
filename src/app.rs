use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{Ctx, Fonts, GpuContext, InputState, MouseState, ShapeRenderer, TextRenderer};

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn update(&mut self, ctx: &mut Ctx);
}

pub fn run<A: App>(title: &str, width: u32, height: u32) {
    EventLoop::new()
        .unwrap()
        .run_app(&mut Runner::new(A::new(), title, width, height))
        .unwrap();
}

// internal runner

struct Runner<A: App> {
    app: A,
    title: String,
    width: u32,
    height: u32,

    // populated after window creation
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    scale_factor: f64,
    text_renderer: Option<TextRenderer>,
    shape_renderer: Option<ShapeRenderer>,
    fonts: Option<Fonts>,
    mouse: MouseState,
    input: InputState,
}

impl<A: App> Runner<A> {
    fn new(app: A, title: &str, width: u32, height: u32) -> Self {
        Self {
            app,
            title: title.to_string(),
            width,
            height,
            window: None,
            gpu: None,
            scale_factor: 1.0,
            text_renderer: None,
            shape_renderer: None,
            fonts: None,
            mouse: MouseState::default(),
            input: InputState::default(),
        }
    }

    fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    fn gpu(&self) -> &GpuContext {
        self.gpu.as_ref().unwrap()
    }

    fn gpu_mut(&mut self) -> &mut GpuContext {
        self.gpu.as_mut().unwrap()
    }

    fn logical_size(&self) -> (f32, f32) {
        let gpu = self.gpu();
        (
            (gpu.config.width as f64 / self.scale_factor) as f32,
            (gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    fn on_resize(&mut self, width: f32, height: f32) {
        if let Some(tr) = self.text_renderer.as_mut() {
            tr.resize(width, height, self.scale_factor);
        }
        if let Some(sr) = self.shape_renderer.as_mut() {
            sr.resize(width, height);
        }
    }

    fn render(&mut self) {
        println!("render");

        let frame = match self.gpu_mut().begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view, msaa_view) = frame.begin();
        let (width, height) = self.logical_size();

        // let user queue draw calls
        {
            let tr = self.text_renderer.as_mut().unwrap();
            let sr = self.shape_renderer.as_mut().unwrap();
            let fonts = self.fonts.as_mut().unwrap();

            let mut ctx = Ctx::new(&self.mouse, &self.input, fonts, tr, sr, width, height);

            self.app.update(&mut ctx);
        }

        // submit queued draw calls to the gpu
        {
            let gpu = self.gpu.as_ref().unwrap();

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

            self.shape_renderer
                .as_mut()
                .unwrap()
                .render(&gpu.device, &gpu.queue, &mut pass);

            self.text_renderer.as_mut().unwrap().render(
                &mut self.fonts.as_mut().unwrap().font_system,
                width,
                height,
                self.scale_factor,
                &gpu.device,
                &gpu.queue,
                &mut pass,
            );
        }

        self.shape_renderer.as_mut().unwrap().clear();
        self.text_renderer.as_mut().unwrap().clear();
        self.text_renderer.as_mut().unwrap().trim_atlas();

        finisher.present(encoder, &self.gpu().queue);
    }
}

// winit event handling

impl<A: App> ApplicationHandler for Runner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height)),
                )
                .unwrap(),
        );

        self.scale_factor = window.scale_factor();
        self.gpu = Some(pollster::block_on(GpuContext::new(window.clone())));
        self.window = Some(window);

        let (w, h, format) = {
            let gpu = self.gpu();
            let w = (gpu.config.width as f64 / self.scale_factor) as f32;
            let h = (gpu.config.height as f64 / self.scale_factor) as f32;
            (w, h, gpu.format)
        };

        {
            let gpu = self.gpu.as_ref().unwrap();
            let mut text_renderer = TextRenderer::new(&gpu.device, &gpu.queue, format);
            text_renderer.resize(w, h, self.scale_factor);
            let shape_renderer = ShapeRenderer::new(&gpu.device, format, w, h);
            self.text_renderer = Some(text_renderer);
            self.shape_renderer = Some(shape_renderer);
        }

        let mut fonts = Fonts::new();
        fonts.add("default", "Arial", 14.0);
        self.fonts = Some(fonts);

        self.window().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if self.window.is_none() {
            return;
        }

        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let x = (position.x / self.scale_factor) as f32;
                let y = (position.y / self.scale_factor) as f32;
                self.mouse.dx = x - self.mouse.x;
                self.mouse.dy = y - self.mouse.y;
                self.mouse.x = x;
                self.mouse.y = y;
                self.window().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.mouse.left_just_pressed = pressed && !self.mouse.left_pressed;
                        self.mouse.left_just_released = !pressed && self.mouse.left_pressed;
                        self.mouse.left_pressed = pressed;
                    }
                    MouseButton::Right => {
                        self.mouse.right_just_pressed = pressed && !self.mouse.right_pressed;
                        self.mouse.right_just_released = !pressed && self.mouse.right_pressed;
                        self.mouse.right_pressed = pressed;
                    }
                    MouseButton::Middle => {
                        self.mouse.middle_just_pressed = pressed && !self.mouse.middle_pressed;
                        self.mouse.middle_just_released = !pressed && self.mouse.middle_pressed;
                        self.mouse.middle_pressed = pressed;
                    }
                    _ => {}
                }
                self.window().request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.mouse.scroll_x = x;
                        self.mouse.scroll_y = y;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.mouse.scroll_x = pos.x as f32;
                        self.mouse.scroll_y = pos.y as f32;
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if event.state == ElementState::Pressed {
                        self.input.keys_pressed.insert(key);
                        self.input.keys_just_pressed.insert(key);
                    } else {
                        self.input.keys_pressed.remove(&key);
                        self.input.keys_just_released.insert(key);
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => for _c in text.chars() {},
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
                let size = self.window().inner_size();
                self.gpu_mut().resize(size.width, size.height);
                let (w, h) = self.logical_size();
                self.on_resize(w, h);
                self.window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.gpu_mut().resize(size.width, size.height);
                let (w, h) = self.logical_size();
                self.on_resize(w, h);
                self.window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.render();

                self.mouse.left_just_pressed = false;
                self.mouse.left_just_released = false;
                self.mouse.right_just_pressed = false;
                self.mouse.right_just_released = false;
                self.mouse.middle_just_pressed = false;
                self.mouse.middle_just_released = false;
                self.mouse.scroll_x = 0.0;
                self.mouse.scroll_y = 0.0;
                self.input.keys_just_pressed.clear();
                self.input.keys_just_released.clear();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

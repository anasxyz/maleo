use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{Ctx, Fonts, GpuContext, ShapeRenderer, TextRenderer};

pub trait BentoApp: 'static {
    fn once(&mut self, ctx: &mut Ctx);
    fn update(&mut self, ctx: &mut Ctx);
}

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

    fn on_resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, ctx: &mut Ctx) {
        self.gpu.resize(new_size.width, new_size.height);
        let (w, h) = self.logical_size();
        ctx.shape_renderer.resize(w, h);
        ctx.text_renderer.resize(w, h, self.scale_factor);
    }

    fn on_scale_change(
        &mut self,
        scale_factor: f64,
        new_inner_size: winit::dpi::PhysicalSize<u32>,
        ctx: &mut Ctx,
    ) {
        self.scale_factor = scale_factor;
        self.gpu.resize(new_inner_size.width, new_inner_size.height);
        let (w, h) = self.logical_size();
        ctx.shape_renderer.resize(w, h);
        ctx.text_renderer.resize(w, h, self.scale_factor);
    }

    fn render<T: BentoApp>(&mut self, ctx: &mut Ctx, app: &mut T) {
        println!("render");
        ctx.shape_renderer.clear();
        ctx.text_renderer.clear();

        // draw all stored rects
        for rect in &ctx.rects {
            ctx.shape_renderer.rect(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                rect.color.to_array(),
                rect.outline_color.to_array(),
                rect.outline_thickness,
            );
        }

        // draw all stored texts
        for text in &ctx.texts {
            ctx.text_renderer.draw(
                &mut ctx.fonts.font_system,
                text.font_family.clone(),
                text.font_size,
                &text.text,
                text.x,
                text.y,
            );
        }

        app.update(ctx);

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
            ctx.shape_renderer
                .render(&self.gpu.device, &self.gpu.queue, &mut pass);
            ctx.text_renderer.render(
                &mut ctx.fonts.font_system,
                width,
                height,
                self.scale_factor,
                &self.gpu.device,
                &self.gpu.queue,
                &mut pass,
            );
        }

        ctx.text_renderer.trim_atlas();
        finisher.present(encoder, &self.gpu.queue);
    }
}

struct WinitHandler<T: BentoApp> {
    title: String,
    width: u32,
    height: u32,
    app: T,
    ctx: Option<Ctx>,
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
            ctx: None,
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

        let fonts = Fonts::new();
        let mut ctx = Ctx::new(fonts, text_renderer, shape_renderer);
        self.app.once(&mut ctx);
        self.ctx = Some(ctx);
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
        let ctx = match self.ctx.as_mut() {
            Some(ctx) => ctx,
            None => return,
        };

        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let new_x = (position.x / ws.scale_factor) as f32;
                let new_y = (position.y / ws.scale_factor) as f32;
                ctx.mouse.dx = new_x - ctx.mouse.x;
                ctx.mouse.dy = new_y - ctx.mouse.y;
                ctx.mouse.x = new_x;
                ctx.mouse.y = new_y;

                let mouse_snap = ctx.mouse;
                if ctx.exit {
                    self.window_state = None;
                    event_loop.exit();
                    return;
                }
                ctx.mouse.dx = 0.0;
                ctx.mouse.dy = 0.0;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        ctx.mouse.left_just_pressed = pressed && !ctx.mouse.left_pressed;
                        ctx.mouse.left_just_released = !pressed && ctx.mouse.left_pressed;
                        ctx.mouse.left_pressed = pressed;
                    }
                    MouseButton::Right => {
                        ctx.mouse.right_just_pressed = pressed && !ctx.mouse.right_pressed;
                        ctx.mouse.right_just_released = !pressed && ctx.mouse.right_pressed;
                        ctx.mouse.right_pressed = pressed;
                    }
                    MouseButton::Middle => {
                        ctx.mouse.middle_just_pressed = pressed && !ctx.mouse.middle_pressed;
                        ctx.mouse.middle_just_released = !pressed && ctx.mouse.middle_pressed;
                        ctx.mouse.middle_pressed = pressed;
                    }
                    _ => {}
                }
                let mouse_snap = ctx.mouse;

                self.app.update(ctx);
                ws.window.request_redraw();

                if ctx.exit {
                    self.window_state = None;
                    event_loop.exit();
                    return;
                }
                ctx.mouse.left_just_pressed = false;
                ctx.mouse.left_just_released = false;
                ctx.mouse.right_just_pressed = false;
                ctx.mouse.right_just_released = false;
                ctx.mouse.middle_just_pressed = false;
                ctx.mouse.middle_just_released = false;
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        ctx.mouse.scroll_x = x;
                        ctx.mouse.scroll_y = y;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        ctx.mouse.scroll_x = pos.x as f32;
                        ctx.mouse.scroll_y = pos.y as f32;
                    }
                }
                let mouse_snap = ctx.mouse;

                self.app.update(ctx);
                ws.window.request_redraw();

                if ctx.exit {
                    self.window_state = None;
                    event_loop.exit();
                    return;
                }
                ctx.mouse.scroll_x = 0.0;
                ctx.mouse.scroll_y = 0.0;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                if let PhysicalKey::Code(key) = event.physical_key {
                    if pressed {
                        ctx.input.keys_just_pressed.insert(key);
                        ctx.input.keys_pressed.insert(key);
                    } else {
                        ctx.input.keys_just_released.insert(key);
                        ctx.input.keys_pressed.remove(&key);
                    }
                }
                let mouse_snap = ctx.mouse;

                self.app.update(ctx);
                ws.window.request_redraw();

                if ctx.exit {
                    self.window_state = None;
                    event_loop.exit();
                    return;
                }
                ctx.input.keys_just_pressed.clear();
                ctx.input.keys_just_released.clear();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                for c in text.chars() {}
                if ctx.exit {
                    self.window_state = None;
                    event_loop.exit();
                    return;
                }
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => {
                let new_inner = ws.window.inner_size();
                ws.on_scale_change(scale_factor, new_inner, ctx);
                ctx.mark_dirty();
                ws.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                ws.on_resize(new_size, ctx);
                ctx.mark_dirty();
                ws.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if ctx.take_dirty() {
                    ws.render(ctx, &mut self.app);
                }
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

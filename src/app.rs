use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{Ctx, Fonts, GpuContext, ShapeRenderer, TextRenderer, Drawer};

pub trait RentexApp: 'static {
    fn setup(&mut self, ctx: &mut Ctx);
    fn update(&mut self, ctx: &mut Ctx);
}

struct WindowState {
    window: Arc<Window>,
    gpu: GpuContext,
    text_renderer: TextRenderer,
    shape_renderer: ShapeRenderer,
    scale_factor: f64,
}

impl WindowState {
    async fn new(window: Arc<Window>) -> Self {
        let gpu = GpuContext::new(window.clone()).await;
        let scale_factor = window.scale_factor();
        let physical = window.inner_size();
        let width = (physical.width as f64 / scale_factor) as f32;
        let height = (physical.height as f64 / scale_factor) as f32;

        let mut text_renderer = TextRenderer::new(&gpu.device, &gpu.queue, gpu.format);
        let shape_renderer = ShapeRenderer::new(&gpu.device, gpu.format, width, height);
        text_renderer.resize(width, height, scale_factor);

        Self { window, gpu, text_renderer, shape_renderer, scale_factor }
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
        self.shape_renderer.resize(w, h);
        self.text_renderer.resize(w, h, self.scale_factor);
    }

    fn on_scale_change(&mut self, scale_factor: f64, new_inner_size: winit::dpi::PhysicalSize<u32>) {
        self.scale_factor = scale_factor;
        self.gpu.resize(new_inner_size.width, new_inner_size.height);
        let (w, h) = self.logical_size();
        self.shape_renderer.resize(w, h);
        self.text_renderer.resize(w, h, self.scale_factor);
    }

    fn render(&mut self, ctx: &mut Ctx) {
        println!("render");
        self.shape_renderer.clear();
        self.text_renderer.clear();

        let mut drawer = Drawer::new(&mut self.text_renderer, &mut self.shape_renderer, &mut ctx.fonts);
        ctx.widgets.render_all(&mut drawer);

        ctx.layout.compute_all(&mut ctx.widgets.widgets);

        self.shape_renderer.clear();
        self.text_renderer.clear();
        let mut drawer = Drawer::new(&mut self.text_renderer, &mut self.shape_renderer, &mut ctx.fonts);
        ctx.widgets.render_all(&mut drawer);

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
            self.shape_renderer.render(&self.gpu.device, &self.gpu.queue, &mut pass);
            self.text_renderer.render(
                &mut ctx.fonts.font_system,
                width,
                height,
                self.scale_factor,
                &self.gpu.device,
                &self.gpu.queue,
                &mut pass,
            );
        }

        self.text_renderer.trim_atlas();
        finisher.present(encoder, &self.gpu.queue);
    }
}

struct WinitHandler<T: RentexApp> {
    title: String,
    width: u32,
    height: u32,
    app: T,
    ctx: Option<Ctx>,
    window_state: Option<WindowState>,
    setup_done: bool,
}

impl<T: RentexApp> WinitHandler<T> {
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

impl<T: RentexApp> ApplicationHandler for WinitHandler<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window_state.is_some() {
            return;
        }

        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        let ws = pollster::block_on(WindowState::new(window));
        self.window_state = Some(ws);

        let fonts = Fonts::new();
        let mut ctx = Ctx::new(fonts);
        self.app.setup(&mut ctx);
        ctx.widgets.mark_dirty();
        self.ctx = Some(ctx);
        self.setup_done = true;

        self.window_state.as_ref().unwrap().window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
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
                let widget_dirty = ctx.widgets.update_all(&mouse_snap);
                self.app.update(ctx);
                if ctx.exit { self.window_state = None; event_loop.exit(); return; }
                if widget_dirty || ctx.widgets.take_dirty() {
                    ws.window.request_redraw();
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
                let widget_dirty = ctx.widgets.update_all(&mouse_snap);
                self.app.update(ctx);
                if ctx.exit { self.window_state = None; event_loop.exit(); return; }
                if widget_dirty || ctx.widgets.take_dirty() {
                    ws.window.request_redraw();
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
                let widget_dirty = ctx.widgets.update_all(&mouse_snap);
                self.app.update(ctx);
                if ctx.exit { self.window_state = None; event_loop.exit(); return; }
                if widget_dirty || ctx.widgets.take_dirty() {
                    ws.window.request_redraw();
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
                ctx.widgets.update_all(&mouse_snap);
                self.app.update(ctx);
                if ctx.exit { self.window_state = None; event_loop.exit(); return; }
                if ctx.widgets.take_dirty() {
                    ws.window.request_redraw();
                }
                ctx.input.keys_just_pressed.clear();
                ctx.input.keys_just_released.clear();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer: _ } => {
                let new_inner = ws.window.inner_size();
                ws.on_scale_change(scale_factor, new_inner);
                ws.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                ws.on_resize(new_size);
                ws.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                ws.render(ctx);
            }
            WindowEvent::CloseRequested => {
                self.window_state = None;
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
        Self { title: title.to_string(), width, height }
    }

    pub fn run<T: RentexApp>(self, fonts: Fonts, app: T) {
        let event_loop = EventLoop::new().unwrap();
        let mut handler = WinitHandler::new(&self.title, self.width, self.height, app);
        event_loop.run_app(&mut handler).unwrap();
    }
}

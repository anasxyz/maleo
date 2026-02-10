use std::sync::Arc;
use winit::{
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowBuilder},
};

use crate::{Ctx, Fonts, GpuContext, ShapeRenderer, TextRenderer, Drawer};

/// Implement this trait to define your application.
pub trait RentexApp: 'static {
    /// Called once at startup. Create and configure all your widgets here.
    fn setup(&mut self, ctx: &mut Ctx);

    /// Called on every input event where something changed.
    /// Read widget state and mutate your app state here.
    fn update(&mut self, ctx: &mut Ctx);
}

pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Arc<Window>,
    gpu: GpuContext,
    scale_factor: f64,
}

impl App {
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
        Self { event_loop: Some(event_loop), window, gpu, scale_factor }
    }

    #[inline(always)]
    fn logical_size(&self) -> (f32, f32) {
        (
            (self.gpu.config.width as f64 / self.scale_factor) as f32,
            (self.gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    pub fn run<T: RentexApp>(mut self, fonts: Fonts, mut app: T) {
        let (width, height) = self.logical_size();

        let mut text_renderer = TextRenderer::new(&self.gpu.device, &self.gpu.queue, self.gpu.format);
        let mut shape_renderer = ShapeRenderer::new(&self.gpu.device, self.gpu.format, width, height);
        text_renderer.resize(width, height, self.scale_factor);

        let mut ctx = Ctx::new(fonts);

        app.setup(&mut ctx);
        // dirty on first frame so initial layout renders
        ctx.widgets.mark_dirty();

        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            let new_x = (position.x / self.scale_factor) as f32;
                            let new_y = (position.y / self.scale_factor) as f32;
                            ctx.mouse.dx = new_x - ctx.mouse.x;
                            ctx.mouse.dy = new_y - ctx.mouse.y;
                            ctx.mouse.x = new_x;
                            ctx.mouse.y = new_y;

                            // snapshot mouse so we can pass it to update_all
                            // while ctx is still available for app.update
                            let mouse_snap = ctx.mouse;
                            let widget_dirty = ctx.widgets.update_all(&mouse_snap);
                            app.update(&mut ctx);
                            if widget_dirty || ctx.widgets.take_dirty() {
                                self.window.request_redraw();
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
                            app.update(&mut ctx);
                            if widget_dirty || ctx.widgets.take_dirty() {
                                self.window.request_redraw();
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
                            app.update(&mut ctx);
                            if widget_dirty || ctx.widgets.take_dirty() {
                                self.window.request_redraw();
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
                            app.update(&mut ctx);
                            if ctx.widgets.take_dirty() {
                                self.window.request_redraw();
                            }
                            ctx.input.keys_just_pressed.clear();
                            ctx.input.keys_just_released.clear();
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            self.on_scale_change(&mut text_renderer, &mut shape_renderer, scale_factor);
                        }
                        WindowEvent::Resized(new_size) => {
                            self.on_resize(&mut text_renderer, &mut shape_renderer, new_size);
                        }
                        WindowEvent::RedrawRequested => {
                            self.render(&mut text_renderer, &mut shape_renderer, &mut ctx);
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

    fn on_scale_change(&mut self, text: &mut TextRenderer, shapes: &mut ShapeRenderer, scale_factor: f64) {
        self.scale_factor = scale_factor;
        let physical_size = self.window.inner_size();
        self.gpu.resize(physical_size.width, physical_size.height);
        let (w, h) = self.logical_size();
        shapes.resize(w, h);
        text.resize(w, h, self.scale_factor);
        self.window.request_redraw();
    }

    fn on_resize(&mut self, text: &mut TextRenderer, shapes: &mut ShapeRenderer, new_size: winit::dpi::PhysicalSize<u32>) {
        self.gpu.resize(new_size.width, new_size.height);
        let (w, h) = self.logical_size();
        shapes.resize(w, h);
        text.resize(w, h, self.scale_factor);
        self.window.request_redraw();
    }

    fn render(&mut self, text_renderer: &mut TextRenderer, shape_renderer: &mut ShapeRenderer, ctx: &mut Ctx) {
        shape_renderer.clear();
        text_renderer.clear();

        let mut drawer = Drawer::new(text_renderer, shape_renderer, &mut ctx.fonts);
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
            shape_renderer.render(&self.gpu.device, &self.gpu.queue, &mut pass);
            text_renderer.render(
                &mut ctx.fonts.font_system,
                width,
                height,
                self.scale_factor,
                &self.gpu.device,
                &self.gpu.queue,
                &mut pass,
            );
        }

        // trim atlas after pass is dropped — frees unused glyph cache
        println!("about to trim");
        text_renderer.trim_atlas();

        finisher.present(encoder, &self.gpu.queue);
    }
}

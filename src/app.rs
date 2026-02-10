use std::sync::Arc;
use winit::{
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowBuilder},
};

use crate::widgets::WidgetManager;
use crate::{Fonts, GpuContext, InputState, MouseState, ShapeRenderer, TextRenderer, Ui};

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

        Self {
            event_loop: Some(event_loop),
            window,
            gpu,
            scale_factor,
        }
    }

    #[inline(always)]
    fn logical_size(&self) -> (f32, f32) {
        (
            (self.gpu.config.width as f64 / self.scale_factor) as f32,
            (self.gpu.config.height as f64 / self.scale_factor) as f32,
        )
    }

    pub fn run<F>(mut self, mut fonts: Fonts, mut widgets: WidgetManager, mut update_fn: F)
    where
        F: FnMut(&mut WidgetManager, &MouseState, &InputState) + 'static,
    {
        let (width, height) = self.logical_size();

        let mut text_renderer = TextRenderer::new(&self.gpu.device, &self.gpu.queue, self.gpu.format);
        let mut shape_renderer = ShapeRenderer::new(&self.gpu.device, self.gpu.format, width, height);
        text_renderer.resize(width, height, self.scale_factor);

        let mut mouse = MouseState::default();
        let mut input = InputState::default();

        let event_loop = self.event_loop.take().unwrap();

        let _ = event_loop.run(move |event, target| {
            target.set_control_flow(ControlFlow::Wait);

            match event {
                Event::WindowEvent { event, window_id } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CursorMoved { position, .. } => {
                            let new_x = (position.x / self.scale_factor) as f32;
                            let new_y = (position.y / self.scale_factor) as f32;
                            mouse.dx = new_x - mouse.x;
                            mouse.dy = new_y - mouse.y;
                            mouse.x = new_x;
                            mouse.y = new_y;

                            let widget_dirty = widgets.update_all(&mouse);
                            update_fn(&mut widgets, &mouse, &input);
                            let user_dirty = widgets.take_dirty();
                            if widget_dirty { println!("CursorMoved: widget_dirty"); }
                            if user_dirty { println!("CursorMoved: user_dirty"); }
                            if widget_dirty || user_dirty {
                                self.window.request_redraw();
                            }
                            mouse.dx = 0.0;
                            mouse.dy = 0.0;
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let pressed = state == ElementState::Pressed;
                            match button {
                                MouseButton::Left => {
                                    mouse.left_just_pressed = pressed && !mouse.left_pressed;
                                    mouse.left_just_released = !pressed && mouse.left_pressed;
                                    mouse.left_pressed = pressed;
                                }
                                MouseButton::Right => {
                                    mouse.right_just_pressed = pressed && !mouse.right_pressed;
                                    mouse.right_just_released = !pressed && mouse.right_pressed;
                                    mouse.right_pressed = pressed;
                                }
                                MouseButton::Middle => {
                                    mouse.middle_just_pressed = pressed && !mouse.middle_pressed;
                                    mouse.middle_just_released = !pressed && mouse.middle_pressed;
                                    mouse.middle_pressed = pressed;
                                }
                                _ => {}
                            }
                            let widget_dirty = widgets.update_all(&mouse);
                            update_fn(&mut widgets, &mouse, &input);
                            if widget_dirty || widgets.take_dirty() {
                                self.window.request_redraw();
                            }
                            mouse.left_just_pressed = false;
                            mouse.left_just_released = false;
                            mouse.right_just_pressed = false;
                            mouse.right_just_released = false;
                            mouse.middle_just_pressed = false;
                            mouse.middle_just_released = false;
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            match delta {
                                MouseScrollDelta::LineDelta(x, y) => {
                                    mouse.scroll_x = x;
                                    mouse.scroll_y = y;
                                }
                                MouseScrollDelta::PixelDelta(pos) => {
                                    mouse.scroll_x = pos.x as f32;
                                    mouse.scroll_y = pos.y as f32;
                                }
                            }
                            let widget_dirty = widgets.update_all(&mouse);
                            update_fn(&mut widgets, &mouse, &input);
                            if widget_dirty || widgets.take_dirty() {
                                self.window.request_redraw();
                            }
                            mouse.scroll_x = 0.0;
                            mouse.scroll_y = 0.0;
                        }
                        WindowEvent::KeyboardInput { event, .. } => {
                            let pressed = event.state == ElementState::Pressed;
                            if let PhysicalKey::Code(key) = event.physical_key {
                                if pressed {
                                    input.keys_just_pressed.insert(key);
                                    input.keys_pressed.insert(key);
                                } else {
                                    input.keys_just_released.insert(key);
                                    input.keys_pressed.remove(&key);
                                }
                            }
                            update_fn(&mut widgets, &mouse, &input);
                            if widgets.take_dirty() {
                                self.window.request_redraw();
                            }
                            input.keys_just_pressed.clear();
                            input.keys_just_released.clear();
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            self.on_scale_change(&mut text_renderer, &mut shape_renderer, scale_factor);
                        }
                        WindowEvent::Resized(new_size) => {
                            self.on_resize(&mut text_renderer, &mut shape_renderer, new_size);
                        }
                        WindowEvent::RedrawRequested => {
                            self.render(
                                &mut text_renderer,
                                &mut shape_renderer,
                                &mut fonts,
                                &mut widgets,
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

    fn on_scale_change(
        &mut self,
        text_renderer: &mut TextRenderer,
        shape_renderer: &mut ShapeRenderer,
        scale_factor: f64,
    ) {
        self.scale_factor = scale_factor;
        let physical_size = self.window.inner_size();
        self.gpu.resize(physical_size.width, physical_size.height);
        let (width, height) = self.logical_size();
        shape_renderer.resize(width, height);
        text_renderer.resize(width, height, self.scale_factor);
        self.window.request_redraw();
    }

    fn on_resize(
        &mut self,
        text_renderer: &mut TextRenderer,
        shape_renderer: &mut ShapeRenderer,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        self.gpu.resize(new_size.width, new_size.height);
        let (width, height) = self.logical_size();
        shape_renderer.resize(width, height);
        text_renderer.resize(width, height, self.scale_factor);
        self.window.request_redraw();
    }

    fn render(
        &mut self,
        text_renderer: &mut TextRenderer,
        shape_renderer: &mut ShapeRenderer,
        fonts: &mut Fonts,
        widgets: &mut WidgetManager,
    ) {
        println!("redrawing");

        shape_renderer.clear();
        text_renderer.clear();

        let mut ui = Ui::new(text_renderer, shape_renderer, fonts);
        widgets.render_all(&mut ui);

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
                &mut fonts.font_system,
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

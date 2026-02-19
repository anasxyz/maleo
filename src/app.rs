use std::collections::HashSet;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{
    Color, Element, Events, Fonts, GpuContext, ShapeRenderer, TextRenderer,
};

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn update(&mut self, events: &Events) -> Element;
    fn clear_color(&self) -> Color {
        Color::rgb(0.1, 0.1, 0.12)
    }
}

pub fn run<A: App>(title: &str, width: u32, height: u32) {
    EventLoop::new()
        .unwrap()
        .run_app(&mut Runner::new(A::new(), title, width, height))
        .unwrap();
}

struct Runner<A: App> {
    app: A,
    title: String,
    width: u32,
    height: u32,
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    scale_factor: f64,
    text_renderer: Option<TextRenderer>,
    shape_renderer: Option<ShapeRenderer>,
    fonts: Option<Fonts>,
    events: Events,
    hovered_last_frame: HashSet<usize>,
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
            events: Events::default(),
            hovered_last_frame: HashSet::new(),
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

    fn on_resize(&mut self, w: f32, h: f32) {
        if let Some(tr) = self.text_renderer.as_mut() {
            tr.resize(w, h, self.scale_factor);
        }
        if let Some(sr) = self.shape_renderer.as_mut() {
            sr.resize(w, h);
        }
    }

    fn render(&mut self) {

        let frame = match self.gpu_mut().begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view, msaa_view) = frame.begin();
        let (width, height) = self.logical_size();

        let tree = self.app.update(&self.events);
        self.draw_element(&tree);

        {
            let gpu = self.gpu.as_ref().unwrap();
            let clear = self.app.clear_color();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa_view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: clear.r as f64,
                            g: clear.g as f64,
                            b: clear.b as f64,
                            a: 1.0,
                        }),
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

    fn draw_element(&mut self, el: &Element) {
        match el {
            Element::Empty => {}
            Element::Rect { w, h, color, .. } => {
                self.shape_renderer
                    .as_mut()
                    .unwrap()
                    .draw_rect(0.0, 0.0, *w, *h, color.to_array(), [0.0; 4], 0.0);
            }
            Element::Text { content, color, .. } => {
                let fonts = self.fonts.as_mut().unwrap();
                let font_id = fonts.default();
                let entry = fonts.get(font_id);
                let family = entry.family.clone();
                let size = entry.size;
                self.text_renderer.as_mut().unwrap().draw(
                    &mut self.fonts.as_mut().unwrap().font_system,
                    family,
                    size,
                    content,
                    0.0,
                    0.0,
                    *color,
                );
            }
        }
    }
}

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
                self.events.mouse.dx = x - self.events.mouse.x;
                self.events.mouse.dy = y - self.events.mouse.y;
                self.events.mouse.x = x;
                self.events.mouse.y = y;
                self.window().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.events.mouse.left_just_pressed =
                            pressed && !self.events.mouse.left_pressed;
                        self.events.mouse.left_just_released =
                            !pressed && self.events.mouse.left_pressed;
                        self.events.mouse.left_pressed = pressed;
                    }
                    MouseButton::Right => {
                        self.events.mouse.right_just_pressed =
                            pressed && !self.events.mouse.right_pressed;
                        self.events.mouse.right_just_released =
                            !pressed && self.events.mouse.right_pressed;
                        self.events.mouse.right_pressed = pressed;
                    }
                    MouseButton::Middle => {
                        self.events.mouse.middle_just_pressed =
                            pressed && !self.events.mouse.middle_pressed;
                        self.events.mouse.middle_just_released =
                            !pressed && self.events.mouse.middle_pressed;
                        self.events.mouse.middle_pressed = pressed;
                    }
                    _ => {}
                }
                self.window().request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.events.mouse.scroll_x = x;
                        self.events.mouse.scroll_y = y;
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.events.mouse.scroll_x = pos.x as f32;
                        self.events.mouse.scroll_y = pos.y as f32;
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    if event.state == ElementState::Pressed {
                        self.events.keyboard.pressed.insert(key);
                        self.events.keyboard.just_pressed.insert(key);
                    } else {
                        self.events.keyboard.pressed.remove(&key);
                        self.events.keyboard.just_released.insert(key);
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(_)) => {}
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
                self.events.clear_frame_state();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

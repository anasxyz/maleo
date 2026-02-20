use std::collections::HashSet;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{Color, Element, Events, Font, FontId, Fonts, GpuContext, ShapeRenderer, TextRenderer};

pub struct Settings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub clear_color: Color,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: "Bento UI".to_string(),
            width: 800,
            height: 600,
            clear_color: Color::rgb(0.1, 0.1, 0.12),
        }
    }
}

impl Settings {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }
}

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn update(&mut self, events: &Events) -> Element;
    fn fonts(&self, fonts: &mut Fonts) {}
    fn run(settings: Settings) {
        run::<Self>(settings);
    }
}

fn run<A: App>(settings: Settings) {
    EventLoop::new()
        .unwrap()
        .run_app(&mut Runner::new(A::new(), settings))
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
    clear_color: Color,
}

impl<A: App> Runner<A> {
    fn new(app: A, settings: Settings) -> Self {
        Self {
            app,
            title: settings.title.to_string(),
            width: settings.width,
            height: settings.height,
            window: None,
            gpu: None,
            scale_factor: 1.0,
            text_renderer: None,
            shape_renderer: None,
            fonts: None,
            events: Events::default(),
            clear_color: settings.clear_color,
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

        let mut tree = self.app.update(&self.events);
        measure(&tree, self.fonts.as_mut().unwrap());
        layout(&mut tree, 0.0, 0.0, self.fonts.as_mut().unwrap());
        println!("Rendering...");
        self.draw_element(&tree);

        {
            let gpu = self.gpu.as_ref().unwrap();
            let clear = self.clear_color;
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
            Element::Rect {
                w, h, color, style, ..
            } => {
                self.shape_renderer.as_mut().unwrap().draw_rect(
                    style.x,
                    style.y,
                    *w,
                    *h,
                    color.to_array(),
                    [0.0; 4],
                    0.0,
                );
            }
            Element::Text {
                content,
                color,
                font,
                style,
            } => {
                let fonts = self.fonts.as_mut().unwrap();
                let font_id = match font {
                    Font::Name(name) => fonts.get_by_name(name).or_else(|| fonts.default()),
                    Font::Default => fonts.default(),
                };
                let entry = fonts.get(font_id.unwrap());
                let family = entry.family.clone();
                let size = entry.size;
                self.text_renderer.as_mut().unwrap().draw(
                    &mut self.fonts.as_mut().unwrap().font_system,
                    family,
                    size,
                    content,
                    style.x,
                    style.y,
                    *color,
                );
            }
            Element::Row { children, .. } | Element::Column { children, .. } => {
                for child in children {
                    self.draw_element(child);
                }
            }
        }
    }
}

fn measure(element: &Element, fonts: &mut Fonts) -> (f32, f32) {
    match element {
        Element::Empty => (0.0, 0.0),
        Element::Rect { w, h, .. } => (*w, *h),
        Element::Text { content, font, .. } => {
            let font_id = match font {
                Font::Name(name) => fonts.get_by_name(name).or_else(|| fonts.default()),
                Font::Default => fonts.default(),
            };
            fonts.measure(content, font_id.unwrap())
        }
        Element::Row { children, .. } => {
            let mut total_width: f32 = 0.0;
            let mut max_height: f32 = 0.0;
            for child in children {
                let (cw, ch) = measure(child, fonts);
                total_width += cw;
                max_height = max_height.max(ch);
            }
            (total_width, max_height)
        }
        Element::Column { children, .. } => {
            let mut max_width: f32 = 0.0;
            let mut total_height: f32 = 0.0;
            for child in children {
                let (cw, ch) = measure(child, fonts);
                max_width = max_width.max(cw);
                total_height += ch;
            }
            (max_width, total_height)
        }
    }
}

fn layout(element: &mut Element, x: f32, y: f32, fonts: &mut Fonts) {
    match element {
        Element::Empty => {}
        Element::Rect { style, .. } => {
            style.x = x;
            style.y = y;
        }
        Element::Text { style, .. } => {
            style.x = x;
            style.y = y;
        }
        Element::Row {
            style, children, ..
        } => {
            style.x = x;
            style.y = y;
            let mut cursor_x = x;
            for child in children {
                let (cw, _) = measure(child, fonts);
                layout(child, cursor_x, y, fonts);
                cursor_x += cw;
            }
        }
        Element::Column {
            style, children, ..
        } => {
            style.x = x;
            style.y = y;
            let mut cursor_y = y;
            for child in children {
                let (_, ch) = measure(child, fonts);
                layout(child, x, cursor_y, fonts);
                cursor_y += ch;
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
        // run user fonts function
        self.app.fonts(&mut fonts);
        if fonts.default().is_none() {
            println!("No default font found, adding default font...");
            let default_font_id = fonts.add("default", "Arial", 14.0);
            fonts.set_default(default_font_id);
        }

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
                    let key = crate::key_code_to_key(key);
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

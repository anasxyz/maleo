use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton as WinitMouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::draw::draw;
use crate::events::{Event, Key, MouseButton};
use crate::layout::do_layout;
use crate::{Color, Element, Fonts, GpuContext, ShadowRenderer, ShapeRenderer, TextRenderer};

// settings

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
            clear_color: Color::new(0.1, 0.1, 0.12, 1.0),
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

// app trait

pub trait App: 'static + Sized {
    type Action: Clone + 'static;
    fn new() -> Self;
    fn view(&self) -> Element<Self::Action>;
    fn update(&mut self, action: Self::Action) {}
    fn event(&mut self, event: Event) -> Option<Self::Action> {
        None
    }
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

// runner

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
    shadow_renderer: Option<ShadowRenderer>,
    fonts: Option<Fonts>,
    clear_color: Color,
    // internal mouse state for hit testing
    mouse_x: f32,
    mouse_y: f32,
    mouse_left_pressed: bool,
    mouse_left_just_pressed: bool,
    mouse_right_pressed: bool,
    mouse_right_just_pressed: bool,
    mouse_middle_pressed: bool,
    mouse_middle_just_pressed: bool,
    // modifier state baked into key events
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl<A: App> Runner<A> {
    fn new(app: A, settings: Settings) -> Self {
        Self {
            app,
            title: settings.title,
            width: settings.width,
            height: settings.height,
            window: None,
            gpu: None,
            scale_factor: 1.0,
            text_renderer: None,
            shape_renderer: None,
            shadow_renderer: None,
            fonts: None,
            clear_color: settings.clear_color,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left_pressed: false,
            mouse_left_just_pressed: false,
            mouse_right_pressed: false,
            mouse_right_just_pressed: false,
            mouse_middle_pressed: false,
            mouse_middle_just_pressed: false,
            ctrl: false,
            shift: false,
            alt: false,
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

    fn resize_shadow(&mut self, w: f32, h: f32) {
        let gpu = self.gpu.as_ref().unwrap();
        if let Some(sr) = self.shadow_renderer.as_mut() {
            sr.resize(&gpu.device, &gpu.queue, w, h);
        }
    }

    fn dispatch(&mut self, event: Event) {
        if let Some(action) = self.app.event(event) {
            self.app.update(action);
            self.window().request_redraw();
        }
    }

    fn render(&mut self) {
        let frame = match self.gpu_mut().begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view, msaa_view) = frame.begin();
        let (width, height) = self.logical_size();

        let mut tree = self.app.view();
        do_layout(&mut tree, width, height, self.fonts.as_mut().unwrap());

        let actions = draw(
            &mut tree,
            self.shape_renderer.as_mut().unwrap(),
            self.shadow_renderer.as_mut().unwrap(),
            self.text_renderer.as_mut().unwrap(),
            self.fonts.as_mut().unwrap(),
            self.mouse_x,
            self.mouse_y,
            self.mouse_left_just_pressed,
        );

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

            self.shadow_renderer
                .as_mut()
                .unwrap()
                .render(&gpu.device, &gpu.queue, &mut pass);
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

        self.shadow_renderer.as_mut().unwrap().clear();
        self.shape_renderer.as_mut().unwrap().clear();
        self.text_renderer.as_mut().unwrap().clear();
        self.text_renderer.as_mut().unwrap().trim_atlas();
        finisher.present(encoder, &self.gpu().queue);

        for action in actions {
            self.app.update(action);
        }

        self.mouse_left_just_pressed = false;
        self.mouse_right_just_pressed = false;
        self.mouse_middle_just_pressed = false;
    }
}

// winit ApplicationHandler

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
            let shadow_renderer = ShadowRenderer::new(&gpu.device, &gpu.queue, format, w, h);
            self.text_renderer = Some(text_renderer);
            self.shape_renderer = Some(shape_renderer);
            self.shadow_renderer = Some(shadow_renderer);
        }

        let mut fonts = Fonts::new();
        self.app.fonts(&mut fonts);
        if fonts.default.is_none() {
            fonts.add("default", "Arial", 14.0).default();
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
                let dx = x - self.mouse_x;
                let dy = y - self.mouse_y;
                self.mouse_x = x;
                self.mouse_y = y;
                self.dispatch(Event::MouseMoved { x, y, dx, dy });
                self.window().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                let btn = match button {
                    WinitMouseButton::Left => Some(MouseButton::Left),
                    WinitMouseButton::Right => Some(MouseButton::Right),
                    WinitMouseButton::Middle => Some(MouseButton::Middle),
                    _ => None,
                };
                if let Some(btn) = btn {
                    match btn {
                        MouseButton::Left => {
                            self.mouse_left_just_pressed = pressed && !self.mouse_left_pressed;
                            self.mouse_left_pressed = pressed;
                        }
                        MouseButton::Right => {
                            self.mouse_right_just_pressed = pressed && !self.mouse_right_pressed;
                            self.mouse_right_pressed = pressed;
                        }
                        MouseButton::Middle => {
                            self.mouse_middle_just_pressed = pressed && !self.mouse_middle_pressed;
                            self.mouse_middle_pressed = pressed;
                        }
                    }
                    let x = self.mouse_x;
                    let y = self.mouse_y;
                    if pressed {
                        self.dispatch(Event::MousePressed { button: btn, x, y });
                    } else {
                        self.dispatch(Event::MouseReleased { button: btn, x, y });
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, y),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.dispatch(Event::MouseScrolled { x, y });
                self.window().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    let key = crate::key_code_to_key(key_code);
                    let pressed = event.state == ElementState::Pressed;
                    match key {
                        Key::LControl | Key::RControl => self.ctrl = pressed,
                        Key::LShift | Key::RShift => self.shift = pressed,
                        Key::LAlt | Key::RAlt => self.alt = pressed,
                        _ => {}
                    }
                    let (ctrl, shift, alt) = (self.ctrl, self.shift, self.alt);
                    if pressed {
                        self.dispatch(Event::KeyPressed {
                            key,
                            ctrl,
                            shift,
                            alt,
                        });
                    } else {
                        self.dispatch(Event::KeyReleased {
                            key,
                            ctrl,
                            shift,
                            alt,
                        });
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::Focused(focused) => {
                if !focused {
                    self.ctrl = false;
                    self.shift = false;
                    self.alt = false;
                }
                self.dispatch(if focused {
                    Event::Focused
                } else {
                    Event::Unfocused
                });
                self.window().request_redraw();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(_)) => {}
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor;
                let size = self.window().inner_size();
                self.gpu_mut().resize(size.width, size.height);
                let (w, h) = self.logical_size();
                self.on_resize(w, h);
                self.resize_shadow(w, h);
                self.dispatch(Event::ScaleChanged(scale_factor));
                self.window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.gpu_mut().resize(size.width, size.height);
                let (w, h) = self.logical_size();
                self.on_resize(w, h);
                self.resize_shadow(w, h);
                self.dispatch(Event::Resized {
                    width: w,
                    height: h,
                });
                self.window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            WindowEvent::CloseRequested => {
                self.dispatch(Event::CloseRequested);
                event_loop.exit();
            }
            _ => {}
        }
    }
}

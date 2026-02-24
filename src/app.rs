use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton as WinitMouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::draw::{Cursor, MouseState, draw};
use crate::events::{Event, Key, Modifiers, MouseButton};
use crate::layout::do_layout;
use crate::state::StateStore;
use crate::task::Task;
use crate::widgets::text_editor as te;
use crate::widgets::text_input as ti;
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
    type Action: Clone + Send + 'static;

    fn new() -> Self;
    fn view(&self) -> Element<Self::Action>;
    fn update(&mut self, action: Self::Action) -> Vec<Task<Self::Action>> {
        vec![]
    }
    fn start(&mut self) -> Vec<Task<Self::Action>> {
        vec![]
    }
    fn event(&mut self, event: Event) -> Option<Self::Action> {
        None
    }
    fn fonts(&self, fonts: &mut Fonts) {}
    fn run(settings: Settings) {
        run::<Self>(settings);
    }
}

#[derive(Debug)]
struct Wake;

fn run<A: App>(settings: Settings) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let event_loop = EventLoop::<Wake>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();
    let (tx, rx) = unbounded_channel::<A::Action>();

    event_loop
        .run_app(&mut Runner::new(A::new(), settings, proxy, tx, rx))
        .unwrap();
}

// gfx
// all rendering resources, initialised on first resumed() call

struct Gfx {
    window: Arc<Window>,
    gpu: GpuContext,
    scale_factor: f64,
    text_renderer: TextRenderer,
    shape_renderer: ShapeRenderer,
    shadow_renderer: ShadowRenderer,
    fonts: Fonts,
    clear_color: Color,
    current_cursor: Cursor,
}

impl Gfx {
    fn logical_size(&self) -> (f32, f32) {
        let w = (self.gpu.config.width as f64 / self.scale_factor) as f32;
        let h = (self.gpu.config.height as f64 / self.scale_factor) as f32;
        (w, h)
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.gpu.resize(width, height);
        let (w, h) = self.logical_size();
        self.text_renderer.resize(w, h, self.scale_factor);
        self.shape_renderer.resize(w, h);
        self.shadow_renderer
            .resize(&self.gpu.device, &self.gpu.queue, w, h);
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        if cursor == self.current_cursor {
            return;
        }
        self.current_cursor = cursor;
        let icon = match cursor {
            Cursor::Default => winit::window::CursorIcon::Default,
            Cursor::Text => winit::window::CursorIcon::Text,
            Cursor::Pointer => winit::window::CursorIcon::Pointer,
            Cursor::Crosshair => winit::window::CursorIcon::Crosshair,
            Cursor::Move => winit::window::CursorIcon::Move,
            Cursor::ResizeNS => winit::window::CursorIcon::NsResize,
            Cursor::ResizeEW => winit::window::CursorIcon::EwResize,
            Cursor::NotAllowed => winit::window::CursorIcon::NotAllowed,
            Cursor::Grab => winit::window::CursorIcon::Grab,
            Cursor::Grabbing => winit::window::CursorIcon::Grabbing,
            Cursor::Wait => winit::window::CursorIcon::Wait,
        };
        self.window.set_cursor(icon);
    }
}

// Tasks — async task machinery

struct Tasks<Action: Clone + Send + 'static> {
    tx: UnboundedSender<Action>,
    rx: UnboundedReceiver<Action>,
    proxy: EventLoopProxy<Wake>,
    exclusive: HashMap<u64, tokio::task::AbortHandle>,
}

impl<Action: Clone + Send + 'static> Tasks<Action> {
    fn new(
        tx: UnboundedSender<Action>,
        rx: UnboundedReceiver<Action>,
        proxy: EventLoopProxy<Wake>,
    ) -> Self {
        Self {
            tx,
            rx,
            proxy,
            exclusive: HashMap::new(),
        }
    }

    fn spawn(&mut self, tasks: Vec<Task<Action>>) {
        for task in tasks {
            let exclusive_key = task.exclusive_key;
            if let Some(key) = exclusive_key {
                if let Some(old) = self.exclusive.remove(&key) {
                    old.abort();
                }
            }
            let tx = self.tx.clone();
            let proxy = self.proxy.clone();
            let send = Arc::new(move |action| {
                let _ = tx.send(action);
                let _ = proxy.send_event(Wake);
            });
            task.spawn(send, |handle| {
                if let Some(key) = exclusive_key {
                    self.exclusive.insert(key, handle);
                }
            });
        }
    }

    fn drain(&mut self) -> Vec<Action> {
        let mut actions = vec![];
        while let Ok(action) = self.rx.try_recv() {
            actions.push(action);
        }
        actions
    }
}

// which text widget currently has keyboard focus
enum FocusedWidget {
    TextInput(String),
    TextEditor(String),
}

// runner

struct Runner<A: App> {
    app: A,
    gfx: Option<Gfx>,
    init: Settings,
    state: StateStore,
    mouse: MouseState,
    modifiers: Modifiers,
    focused_widget: Option<FocusedWidget>,
    tasks: Tasks<A::Action>,
}

impl<A: App> Runner<A> {
    fn new(
        app: A,
        settings: Settings,
        proxy: EventLoopProxy<Wake>,
        tx: UnboundedSender<A::Action>,
        rx: UnboundedReceiver<A::Action>,
    ) -> Self {
        Self {
            app,
            gfx: None,
            init: settings,
            state: StateStore::new(),
            mouse: MouseState {
                x: 0.0,
                y: 0.0,
                left_pressed: false,
                left_just_pressed: false,
                left_just_released: false,
                right_pressed: false,
                right_just_pressed: false,
                middle_pressed: false,
                middle_just_pressed: false,
                left_click_count: 0,
                left_click_x: 0.0,
                left_click_y: 0.0,
                click_timer: std::time::Instant::now(),
                last_click_time: -1.0,
            },
            modifiers: Modifiers::default(),
            focused_widget: None,
            tasks: Tasks::new(tx, rx, proxy),
        }
    }

    fn gfx(&self) -> &Gfx {
        self.gfx.as_ref().unwrap()
    }

    fn gfx_mut(&mut self) -> &mut Gfx {
        self.gfx.as_mut().unwrap()
    }

    fn dispatch_event(&mut self, event: Event) {
        if let Some(action) = self.app.event(event) {
            let tasks = self.app.update(action);
            self.tasks.spawn(tasks);
            self.gfx().window.request_redraw();
        }
    }

    fn drain_channel(&mut self) {
        let actions = self.tasks.drain();
        let had_any = !actions.is_empty();
        for action in actions {
            let tasks = self.app.update(action);
            self.tasks.spawn(tasks);
        }
        if had_any {
            self.gfx().window.request_redraw();
        }
    }

    fn render(&mut self) {
        let gfx = self.gfx.as_mut().unwrap();

        let frame = match gfx.gpu.begin_frame() {
            Ok(f) => f,
            Err(_) => return,
        };

        let (mut encoder, finisher, view) = frame.begin();
        let (width, height) = gfx.logical_size();

        let mut tree = self.app.view();
        do_layout(&mut tree, width, height, &mut gfx.fonts);

        let (actions, cursor) = draw(
            &mut tree,
            &mut gfx.shape_renderer,
            &mut gfx.shadow_renderer,
            &mut gfx.text_renderer,
            &mut gfx.fonts,
            &mut self.state,
            &self.mouse,
            gfx.scale_factor as f32,
        );

        gfx.set_cursor(cursor.unwrap_or(Cursor::Default));

        {
            let clear = gfx.clear_color;
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
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

            gfx.shadow_renderer
                .render(&gfx.gpu.device, &gfx.gpu.queue, &mut pass);
            gfx.shape_renderer
                .render(&gfx.gpu.device, &gfx.gpu.queue, &mut pass);
            gfx.text_renderer.render(
                &mut gfx.fonts.font_system,
                width,
                height,
                gfx.scale_factor,
                &gfx.gpu.device,
                &gfx.gpu.queue,
                &mut pass,
            );
        }

        gfx.shadow_renderer.clear();
        gfx.shape_renderer.clear();
        gfx.text_renderer.clear();
        gfx.text_renderer.trim_atlas();
        finisher.present(encoder, &gfx.gpu.queue);

        let had_actions = !actions.is_empty();
        for action in actions {
            let tasks = self.app.update(action);
            self.tasks.spawn(tasks);
        }
        if had_actions {
            self.gfx().window.request_redraw();
        }

        self.focused_widget = ti::find_focused(&self.state)
            .map(FocusedWidget::TextInput)
            .or_else(|| te::find_focused(&self.state).map(FocusedWidget::TextEditor));

        // reset one-frame flags
        self.mouse.left_just_pressed = false;
        self.mouse.left_just_released = false;
        self.mouse.right_just_pressed = false;
        self.mouse.middle_just_pressed = false;
    }
}

impl<A: App> ApplicationHandler<Wake> for Runner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&self.init.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            self.init.width,
                            self.init.height,
                        )),
                )
                .unwrap(),
        );

        let scale_factor = window.scale_factor();
        let gpu = pollster::block_on(GpuContext::new(window.clone()));

        let w = (gpu.config.width as f64 / scale_factor) as f32;
        let h = (gpu.config.height as f64 / scale_factor) as f32;
        let format = gpu.format;

        let mut text_renderer = TextRenderer::new(&gpu.device, &gpu.queue, format);
        text_renderer.resize(w, h, scale_factor);
        let shape_renderer = ShapeRenderer::new(&gpu.device, format, w, h);
        let shadow_renderer = ShadowRenderer::new(&gpu.device, &gpu.queue, format, w, h);

        let mut fonts = Fonts::new();
        self.app.fonts(&mut fonts);
        if fonts.default.is_none() {
            fonts.add("default", "Arial", 14.0).default();
        }

        self.gfx = Some(Gfx {
            window,
            gpu,
            scale_factor,
            text_renderer,
            shape_renderer,
            shadow_renderer,
            fonts,
            clear_color: self.init.clear_color,
            current_cursor: Cursor::Default,
        });

        let tasks = self.app.start();
        self.tasks.spawn(tasks);
        self.gfx().window.request_redraw();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: Wake) {
        self.drain_channel();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if self.gfx.is_none() {
            return;
        }
        event_loop.set_control_flow(ControlFlow::Wait);

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let scale = self.gfx().scale_factor;
                let x = (position.x / scale) as f32;
                let y = (position.y / scale) as f32;
                let dx = x - self.mouse.x;
                let dy = y - self.mouse.y;
                self.mouse.x = x;
                self.mouse.y = y;
                self.dispatch_event(Event::MouseMoved { x, y, dx, dy });
                self.gfx().window.request_redraw();
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
                            self.mouse.left_just_pressed = pressed && !self.mouse.left_pressed;
                            self.mouse.left_just_released = !pressed && self.mouse.left_pressed;
                            self.mouse.left_pressed = pressed;
                            if pressed {
                                let now = self.mouse.click_timer.elapsed().as_secs_f64();
                                let dt = now - self.mouse.last_click_time;
                                let dx = self.mouse.x - self.mouse.left_click_x;
                                let dy = self.mouse.y - self.mouse.left_click_y;
                                let dist = (dx * dx + dy * dy).sqrt();
                                const DOUBLE_CLICK_TIME: f64 = 0.3;
                                const DOUBLE_CLICK_DIST: f32 = 4.0;
                                if dt < DOUBLE_CLICK_TIME && dist < DOUBLE_CLICK_DIST {
                                    self.mouse.left_click_count += 1;
                                } else {
                                    self.mouse.left_click_count = 1;
                                }
                                self.mouse.left_click_x = self.mouse.x;
                                self.mouse.left_click_y = self.mouse.y;
                                self.mouse.last_click_time = now;
                            }
                        }
                        MouseButton::Right => {
                            self.mouse.right_just_pressed = pressed && !self.mouse.right_pressed;
                            self.mouse.right_pressed = pressed;
                        }
                        MouseButton::Middle => {
                            self.mouse.middle_just_pressed = pressed && !self.mouse.middle_pressed;
                            self.mouse.middle_pressed = pressed;
                        }
                    }
                    let x = self.mouse.x;
                    let y = self.mouse.y;
                    if pressed {
                        self.dispatch_event(Event::MousePressed { button: btn, x, y });
                    } else {
                        self.dispatch_event(Event::MouseReleased { button: btn, x, y });
                    }
                }
                self.gfx().window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, y),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.dispatch_event(Event::MouseScrolled { x, y });
                self.gfx().window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    let key = crate::key_code_to_key(key_code);
                    let pressed = event.state == ElementState::Pressed;

                    match key {
                        Key::LControl | Key::RControl => self.modifiers.ctrl = pressed,
                        Key::LShift | Key::RShift => self.modifiers.shift = pressed,
                        Key::LAlt | Key::RAlt => self.modifiers.alt = pressed,
                        _ => {}
                    }

                    let modifiers = self.modifiers;
                    let bento_event = if pressed {
                        Event::KeyPressed { key, modifiers }
                    } else {
                        Event::KeyReleased { key, modifiers }
                    };

                    // app gets first priority, if it handles the event, the widget doesnt see it
                    let app_consumed = if let Some(action) = self.app.event(bento_event.clone()) {
                        let tasks = self.app.update(action);
                        self.tasks.spawn(tasks);
                        self.gfx().window.request_redraw();
                        true
                    } else {
                        false
                    };

                    // if app didnt consume it and a text widget is focused, route to the widget
                    if !app_consumed {
                        if let Some(focused) = &self.focused_widget {
                            let text = if pressed && !self.modifiers.ctrl {
                                event.text.as_ref().map(|t| t.as_str()).unwrap_or("")
                            } else {
                                ""
                            };

                            let result = match focused {
                                FocusedWidget::TextInput(id) => {
                                    let id = id.clone();
                                    ti::handle_key(&mut self.state, &id, &bento_event, text).map(
                                        |v| ti::call_callback::<A::Action>(&self.state, &id, v),
                                    )
                                }
                                FocusedWidget::TextEditor(id) => {
                                    let id = id.clone();
                                    te::handle_key(&mut self.state, &id, &bento_event, text).map(
                                        |v| te::call_callback::<A::Action>(&self.state, &id, v),
                                    )
                                }
                            };

                            if let Some(maybe_action) = result {
                                if let Some(action) = maybe_action {
                                    let tasks = self.app.update(action);
                                    self.tasks.spawn(tasks);
                                }
                                self.gfx().window.request_redraw();
                            }
                        }
                    }
                }
                self.gfx().window.request_redraw();
            }
            WindowEvent::Focused(focused) => {
                if !focused {
                    self.modifiers = Modifiers::default();
                }
                self.dispatch_event(if focused {
                    Event::Focused
                } else {
                    Event::Unfocused
                });
                self.gfx().window.request_redraw();
            }
            WindowEvent::Ime(winit::event::Ime::Commit(_)) => {}
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                let size = self.gfx().window.inner_size();
                self.gfx_mut().scale_factor = scale_factor;
                self.gfx_mut().resize(size.width, size.height);
                self.dispatch_event(Event::ScaleChanged(scale_factor));
                self.gfx().window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.gfx_mut().resize(size.width, size.height);
                let (w, h) = self.gfx().logical_size();
                self.dispatch_event(Event::Resized {
                    width: w,
                    height: h,
                });
                self.gfx().window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.drain_channel();
                self.render();
            }
            WindowEvent::CloseRequested => {
                self.dispatch_event(Event::CloseRequested);
                event_loop.exit();
            }
            _ => {}
        }
    }
}

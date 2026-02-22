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

use crate::draw::{draw, text_input_key};
use crate::events::{Event, Key, MouseButton};
use crate::layout::do_layout;
use crate::state::StateStore;
use crate::task::Task;
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

// user event to wake winit from background threads
#[derive(Debug)]
struct Wake;

fn run<A: App>(settings: Settings) {
    // start tokio runtime on a background thread
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let event_loop = EventLoop::<Wake>::with_user_event().build().unwrap();
    let proxy = event_loop.create_proxy();
    let (tx, rx) = unbounded_channel::<A::Action>();

    event_loop
        .run_app(&mut Runner::new(A::new(), settings, proxy, tx, rx, rt))
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
    state: StateStore,
    focused_input_id: Option<String>,
    clear_color: Color,
    // task infrastructure
    proxy: EventLoopProxy<Wake>,
    tx: UnboundedSender<A::Action>,
    rx: UnboundedReceiver<A::Action>,
    rt: tokio::runtime::Runtime,
    // internal mouse state for hit testing
    mouse_x: f32,
    mouse_y: f32,
    mouse_left_pressed: bool,
    mouse_left_just_pressed: bool,
    mouse_right_pressed: bool,
    mouse_right_just_pressed: bool,
    mouse_middle_pressed: bool,
    mouse_middle_just_pressed: bool,
    // exclusive task tracking — keyed by call site hash
    exclusive_tasks: HashMap<u64, tokio::task::AbortHandle>,
    // modifier state baked into key events
    ctrl: bool,
    shift: bool,
    alt: bool,
}

impl<A: App> Runner<A> {
    fn new(
        app: A,
        settings: Settings,
        proxy: EventLoopProxy<Wake>,
        tx: UnboundedSender<A::Action>,
        rx: UnboundedReceiver<A::Action>,
        rt: tokio::runtime::Runtime,
    ) -> Self {
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
            state: StateStore::new(),
            focused_input_id: None,
            clear_color: settings.clear_color,
            proxy,
            tx,
            rx,
            rt,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left_pressed: false,
            mouse_left_just_pressed: false,
            mouse_right_pressed: false,
            mouse_right_just_pressed: false,
            mouse_middle_pressed: false,
            mouse_middle_just_pressed: false,
            exclusive_tasks: HashMap::new(),
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

    // spawn a vec of tasks — handles exclusive cancellation automatically
    fn spawn_tasks(&mut self, tasks: Vec<Task<A::Action>>) {
        for task in tasks {
            let exclusive_key = task.exclusive_key;

            // if exclusive, cancel any previous task from the same call site
            if let Some(key) = exclusive_key {
                if let Some(old_handle) = self.exclusive_tasks.remove(&key) {
                    old_handle.abort();
                }
            }

            let tx = self.tx.clone();
            let proxy = self.proxy.clone();
            let send = Arc::new(move |action| {
                let _ = tx.send(action);
                let _ = proxy.send_event(Wake);
            });

            let exclusive_tasks = &mut self.exclusive_tasks;
            task.spawn(send, |handle| {
                if let Some(key) = exclusive_key {
                    exclusive_tasks.insert(key, handle);
                }
            });
        }
    }

    fn dispatch_event(&mut self, event: Event) {
        if let Some(action) = self.app.event(event) {
            let tasks = self.app.update(action);
            self.spawn_tasks(tasks);
            self.window().request_redraw();
        }
    }

    // drain the action channel and call update for each pending action
    fn drain_channel(&mut self) {
        let mut any = false;
        while let Ok(action) = self.rx.try_recv() {
            let tasks = self.app.update(action);
            self.spawn_tasks(tasks);
            any = true;
        }
        if any {
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

        // clear callbacks from last frame — draw pass will re-register them
        self.state.clear_text_callbacks();

        let actions = draw(
            &mut tree,
            self.shape_renderer.as_mut().unwrap(),
            self.shadow_renderer.as_mut().unwrap(),
            self.text_renderer.as_mut().unwrap(),
            self.fonts.as_mut().unwrap(),
            &mut self.state,
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

        // dispatch button/widget actions collected during draw
        for action in actions {
            let tasks = self.app.update(action);
            self.spawn_tasks(tasks);
        }

        // update focused input id by scanning state
        self.focused_input_id = crate::draw::find_focused_input(&self.state);

        self.mouse_left_just_pressed = false;
        self.mouse_right_just_pressed = false;
        self.mouse_middle_just_pressed = false;
    }
}

// winit ApplicationHandler

impl<A: App> ApplicationHandler<Wake> for Runner<A> {
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

        // fire start() tasks
        let tasks = self.app.start();
        self.spawn_tasks(tasks);

        self.window().request_redraw();
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: Wake) {
        // a background task finished and sent an action — drain the channel
        self.drain_channel();
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
                self.dispatch_event(Event::MouseMoved { x, y, dx, dy });
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
                        self.dispatch_event(Event::MousePressed { button: btn, x, y });
                    } else {
                        self.dispatch_event(Event::MouseReleased { button: btn, x, y });
                    }
                }
                self.window().request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (x, y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, y),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                self.dispatch_event(Event::MouseScrolled { x, y });
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
                    let bento_event = if pressed {
                        Event::KeyPressed {
                            key,
                            ctrl,
                            shift,
                            alt,
                        }
                    } else {
                        Event::KeyReleased {
                            key,
                            ctrl,
                            shift,
                            alt,
                        }
                    };

                    // forward to focused text input first — consumes the event if handled
                    let mut consumed = false;
                    eprintln!(
                        "KEY: focused_input_id={:?} pressed={}",
                        self.focused_input_id, pressed
                    );
                    if let Some(id) = &self.focused_input_id.clone() {
                        let current_value = self.state.get_input_value(id);
                        let text = if pressed && !ctrl {
                            event.text.as_ref().map(|t| t.as_str()).unwrap_or("")
                        } else {
                            ""
                        };
                        eprintln!(
                            "  id={} current_value={:?} text={:?}",
                            id, current_value, text
                        );
                        if let Some(new_value) =
                            text_input_key(&mut self.state, id, &current_value, &bento_event, text)
                        {
                            eprintln!("  new_value={:?}", new_value);
                            consumed = true;
                            if let Some(action) =
                                self.state.call_text_callback::<A::Action>(id, new_value)
                            {
                                eprintln!("  callback fired");
                                let tasks = self.app.update(action);
                                self.spawn_tasks(tasks);
                            } else {
                                eprintln!("  callback returned None");
                            }
                            self.window().request_redraw();
                        } else if matches!(
                            bento_event,
                            Event::KeyPressed {
                                key: Key::Left
                                    | Key::Right
                                    | Key::Home
                                    | Key::End
                                    | Key::Backspace
                                    | Key::Delete,
                                ..
                            }
                        ) {
                            consumed = true;
                            self.window().request_redraw();
                        }
                    }

                    if !consumed {
                        self.dispatch_event(bento_event);
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
                self.dispatch_event(if focused {
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
                self.dispatch_event(Event::ScaleChanged(scale_factor));
                self.window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                self.gpu_mut().resize(size.width, size.height);
                let (w, h) = self.logical_size();
                self.on_resize(w, h);
                self.resize_shadow(w, h);
                self.dispatch_event(Event::Resized {
                    width: w,
                    height: h,
                });
                self.window().request_redraw();
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

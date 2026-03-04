use pollster;
use std::collections::HashMap;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler, event::{ElementState, WindowEvent}, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowId}
};

use crate::Color;
use crate::GpuContext;
use crate::window::WindowState;
use crate::settings::Settings;

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn run(settings: Settings) {
        run::<Self>(settings);
    }
}

fn run<A: App>(settings: Settings) {
    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut Runner::new(A::new(), settings))
        .unwrap();
}

struct Runner<A: App> {
    app: A,
    windows: HashMap<WindowId, WindowState>,
    init: Settings,
}

impl<A: App> Runner<A> {
    fn new(app: A, settings: Settings) -> Self {
        Self {
            app,
            windows: HashMap::new(),
            init: settings,
        }
    }

    fn open_window(&mut self, event_loop: &ActiveEventLoop, settings: &Settings) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(&settings.title)
                        .with_inner_size(winit::dpi::LogicalSize::new(
                            settings.width,
                            settings.height,
                        )),
                )
                .unwrap(),
        );
        let gpu = pollster::block_on(GpuContext::new(window.clone()));
        let id = window.id();
        self.windows
            .insert(id, WindowState::new(window, gpu, settings.clear_color));
    }
}

impl<A: App> ApplicationHandler for Runner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let settings = self.init.clone();
        self.open_window(event_loop, &settings);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        event_loop.set_control_flow(ControlFlow::Wait);
        let Some(win) = self.windows.get_mut(&id) else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => win.render(),
            WindowEvent::CloseRequested => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let PhysicalKey::Code(KeyCode::KeyL) = event.physical_key {
                        let new_settings = Settings {
                            title: "demo".to_string(),
                            width: 640,
                            height: 480,
                            clear_color: Color::new(0.2, 0.1, 0.1, 1.0),
                        };
                        self.open_window(event_loop, &new_settings);
                    }
                }
            }
            _ => {}
        }
    }
}

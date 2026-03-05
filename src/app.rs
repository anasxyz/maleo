use pollster;
use std::collections::HashMap;
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::element::{ElementType, print_root};
use crate::render::{gpu::GpuContext, shape_renderer::RectParams};
use crate::settings::WindowSettings;
use crate::window::WindowState;
use crate::{color::Color, element::Element};
use crate::layout::layout_tree;

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn view(&mut self) -> Element;
    fn run(settings: WindowSettings) {
        run::<Self>(settings);
    }
}

fn run<A: App>(settings: WindowSettings) {
    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut Runner::new(A::new(), settings))
        .unwrap();
}

struct Runner<A: App> {
    app: A,
    windows: HashMap<WindowId, WindowState>,
    init: WindowSettings,
}

impl<A: App> Runner<A> {
    fn new(app: A, settings: WindowSettings) -> Self {
        Self {
            app,
            windows: HashMap::new(),
            init: settings,
        }
    }

    fn open_window(&mut self, event_loop: &ActiveEventLoop, settings: &WindowSettings) {
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

fn draw_tree(el: &Element, draw: &mut crate::render::draw::DrawContext) {
    match el._type {
        ElementType::Rect => {
            draw.draw_rect(
                el.style.x,
                el.style.y,
                el.style.w,
                el.style.h,
                RectParams {
                    color: el.style.fill.to_array(),
                    radius: el.style.border_radius.unwrap_or(0.0),
                    border_color: el.style.border_color.unwrap_or(Color::BLACK).to_array(),
                    border_width: el.style.border_thickness,
                    clip: None,
                },
            );
        }
        _ => {}
    }
    if let Some(children) = &el.children {
        for child in children {
            draw_tree(child, draw);
        }
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
            WindowEvent::RedrawRequested => {
                let mut element = self.app.view();

                // print whole element tree
                print_root(&element);

                win.begin();

                // drawing happens here
                // ...
                println!("drawing tree");
                layout_tree(&mut element);
                draw_tree(&element, &mut win.draw);

                win.render();
            }
            WindowEvent::Resized(size) => {
                win.gpu.resize(size.width, size.height);
                win.draw.resize(size.width, size.height);
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                win.draw.set_scale(scale_factor as f32);
            }
            WindowEvent::CloseRequested => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    event_loop.exit();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if let PhysicalKey::Code(KeyCode::KeyL) = event.physical_key {
                        let new_settings = WindowSettings {
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

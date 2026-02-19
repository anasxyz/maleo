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
    Color, Element, Events, Fonts, GpuContext, LayoutKind, LayoutNode, ShapeRenderer, TextRenderer,
};

pub trait App: 'static + Sized {
    fn new() -> Self;
    fn update(&mut self, events: &Events) -> Element<Self>;
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

    fn on_resize(&mut self, width: f32, height: f32) {
        if let Some(tr) = self.text_renderer.as_mut() {
            tr.resize(width, height, self.scale_factor);
        }
        if let Some(sr) = self.shape_renderer.as_mut() {
            sr.resize(width, height);
        }
    }

    fn layout(&mut self, element: Element<A>, x: f32, y: f32) -> LayoutNode<A> {
        match element {
            Element::Rect {
                w,
                h,
                color,
                hover_color,
                padding: p,
                callbacks,
            } => {
                let x = x + p.left;
                let y = y + p.top;
                let w = w + p.left + p.right;
                let h = h + p.top + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Rect {
                        color,
                        hover_color,
                        hovered: false,
                        callbacks,
                    },
                }
            }
            Element::Text {
                content,
                color,
                padding: p,
            } => {
                let fonts = self.fonts.as_mut().unwrap();
                let font = fonts.default();
                let (w, h) = fonts.measure(&content, font);
                let x = x + p.left;
                let y = y + p.top;
                let w = w + p.left + p.right;
                let h = h + p.top + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Text { content, color },
                }
            }
            Element::Button {
                label,
                w,
                h,
                style,
                padding: p,
                on_click,
            } => {
                let x = x + p.left;
                let y = y + p.top;
                let w = w + p.left + p.right;
                let h = h + p.top + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Button {
                        label,
                        style,
                        on_click,
                        hovered: false,
                    },
                }
            }
            Element::Row {
                gap,
                padding: p,
                children,
            } => {
                let mut cursor_x = x + p.left;
                let mut max_h = 0.0f32;
                let mut nodes = Vec::with_capacity(children.len());
                for child in children {
                    let node = self.layout(child, cursor_x, y + p.top);
                    cursor_x += node.w + gap;
                    max_h = max_h.max(node.h);
                    nodes.push(node);
                }
                let w = cursor_x - x + p.right;
                let h = max_h + p.top + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Children(nodes),
                }
            }
            Element::Column {
                gap,
                padding: p,
                children,
            } => {
                let mut cursor_y = y + p.top;
                let mut max_w = 0.0f32;
                let mut nodes = Vec::with_capacity(children.len());
                for child in children {
                    let node = self.layout(child, x + p.left, cursor_y);
                    cursor_y += node.h + gap;
                    max_w = max_w.max(node.w);
                    nodes.push(node);
                }
                let w = max_w + p.left + p.right;
                let h = cursor_y - y + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Children(nodes),
                }
            }
            Element::Container {
                color,
                padding: p,
                child,
            } => {
                let inner = self.layout(*child, x + p.left, y + p.top);
                let w = inner.w + p.left + p.right;
                let h = inner.h + p.top + p.bottom;
                LayoutNode {
                    x,
                    y,
                    w,
                    h,
                    kind: LayoutKind::Container {
                        color,
                        child: Box::new(inner),
                    },
                }
            }
            Element::Empty => LayoutNode {
                x,
                y,
                w: 0.0,
                h: 0.0,
                kind: LayoutKind::Empty,
            },
        }
    }

    fn fire_callbacks(
        app: &mut A,
        node: &mut LayoutNode<A>,
        mouse_x: f32,
        mouse_y: f32,
        clicked: bool,
        hovered_last: &HashSet<usize>,
        hovered_this: &mut HashSet<usize>,
        index: &mut usize,
    ) {
        let i = *index;
        *index += 1;

        let over = mouse_x >= node.x
            && mouse_x <= node.x + node.w
            && mouse_y >= node.y
            && mouse_y <= node.y + node.h;

        match &mut node.kind {
            LayoutKind::Rect {
                callbacks, hovered, ..
            } => {
                if over {
                    hovered_this.insert(i);
                    *hovered = true;
                    if let Some(f) = &mut callbacks.on_hover {
                        f(app);
                    }
                    if !hovered_last.contains(&i) {
                        if let Some(f) = &mut callbacks.on_just_hovered {
                            f(app);
                        }
                    }
                    if clicked {
                        if let Some(f) = &mut callbacks.on_click {
                            f(app);
                        }
                    }
                } else {
                    *hovered = false;
                    if hovered_last.contains(&i) {
                        if let Some(f) = &mut callbacks.on_just_unhovered {
                            f(app);
                        }
                    }
                }
            }
            LayoutKind::Button {
                on_click, hovered, ..
            } => {
                if over {
                    hovered_this.insert(i);
                    *hovered = true;
                    if clicked {
                        if let Some(f) = on_click {
                            f(app);
                        }
                    }
                } else {
                    *hovered = false;
                }
            }
            LayoutKind::Container { child, .. } => {
                Self::fire_callbacks(
                    app,
                    child,
                    mouse_x,
                    mouse_y,
                    clicked,
                    hovered_last,
                    hovered_this,
                    index,
                );
            }
            LayoutKind::Children(children) => {
                for child in children {
                    Self::fire_callbacks(
                        app,
                        child,
                        mouse_x,
                        mouse_y,
                        clicked,
                        hovered_last,
                        hovered_this,
                        index,
                    );
                }
            }
            _ => {}
        }
    }

    fn draw_layout(&mut self, node: &LayoutNode<A>) {
        let sr = self.shape_renderer.as_mut().unwrap();
        let tr = self.text_renderer.as_mut().unwrap();
        let fonts = self.fonts.as_mut().unwrap();

        match &node.kind {
            LayoutKind::Rect {
                color,
                hover_color,
                hovered,
                ..
            } => {
                let c = if *hovered {
                    hover_color.unwrap_or(*color)
                } else {
                    *color
                };
                sr.draw_rect(node.x, node.y, node.w, node.h, c.to_array(), [0.0; 4], 0.0);
            }
            LayoutKind::Text { content, color } => {
                let font = fonts.default();
                let entry = fonts.get(font);
                let family = entry.family.clone();
                let size = entry.size;
                tr.draw(
                    &mut fonts.font_system,
                    family,
                    size,
                    content,
                    node.x,
                    node.y,
                    *color,
                );
            }
            LayoutKind::Button {
                label,
                style,
                hovered,
                ..
            } => {
                let color = if *hovered {
                    style.hover_color
                } else {
                    style.color
                };
                sr.draw_rounded_rect(
                    node.x,
                    node.y,
                    node.w,
                    node.h,
                    style.corner_radius,
                    color.to_array(),
                    [0.0; 4],
                    0.0,
                );
                let font = fonts.default();
                let entry = fonts.get(font);
                let family = entry.family.clone();
                let size = entry.size;
                let (tw, th) = fonts.measure(label, font);
                let tx = node.x + (node.w - tw) / 2.0;
                let ty = node.y + (node.h - th) / 2.0;
                tr.draw(
                    &mut fonts.font_system,
                    family,
                    size,
                    label,
                    tx,
                    ty,
                    style.text_color,
                );
            }
            LayoutKind::Container { color, child } => {
                sr.draw_rect(
                    node.x,
                    node.y,
                    node.w,
                    node.h,
                    color.to_array(),
                    [0.0; 4],
                    0.0,
                );
                self.draw_layout(child);
            }
            LayoutKind::Children(children) => {
                for child in children {
                    self.draw_layout(child);
                }
            }
            LayoutKind::Empty => {}
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
        let mut layout = self.layout(tree, 0.0, 0.0);

        let mouse_x = self.events.mouse.x;
        let mouse_y = self.events.mouse.y;
        let clicked = self.events.mouse.left_just_pressed;
        let mut hovered_this_frame = HashSet::new();
        let mut index = 0;
        Self::fire_callbacks(
            &mut self.app,
            &mut layout,
            mouse_x,
            mouse_y,
            clicked,
            &self.hovered_last_frame,
            &mut hovered_this_frame,
            &mut index,
        );
        self.hovered_last_frame = hovered_this_frame;

        self.draw_layout(&layout);

        {
            let gpu = self.gpu.as_ref().unwrap();
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &msaa_view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear({
                            let c = self.app.clear_color();
                            wgpu::Color {
                                r: c.r as f64,
                                g: c.g as f64,
                                b: c.b as f64,
                                a: 1.0,
                            }
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

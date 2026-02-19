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
    Color, Element, Events, Fonts, GpuContext, LayoutKind, LayoutNode, ShapeRenderer, Size,
    TextRenderer,
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

    fn resolve_size(s: &Option<Size>, natural: f32, avail: f32) -> f32 {
        match s {
            None => natural,
            Some(Size::Fixed(v)) => *v,
            Some(Size::Fill) => avail,
            Some(Size::Percent(p)) => avail * p / 100.0,
        }
    }

    fn is_fill(s: &Option<Size>) -> bool {
        matches!(s, Some(Size::Fill))
    }

    fn child_is_fill_w(element: &Element<A>) -> bool {
        match element {
            Element::Rect { width, .. } => Self::is_fill(width),
            Element::Text { width, .. } => Self::is_fill(width),
            Element::Button { width, .. } => Self::is_fill(width),
            Element::Container { width, .. } => Self::is_fill(width),
            Element::Row { width, .. } => Self::is_fill(width),
            Element::Column { width, .. } => Self::is_fill(width),
            _ => false,
        }
    }

    fn layout_measure(&mut self, element: &Element<A>, avail_w: f32, avail_h: f32) -> (f32, f32) {
        match element {
            Element::Rect {
                w,
                h,
                padding: p,
                width,
                height,
                ..
            } => {
                let w = Self::resolve_size(width, *w, avail_w - p.left - p.right);
                let h = Self::resolve_size(height, *h, avail_h - p.top - p.bottom);
                (w + p.left + p.right, h + p.top + p.bottom)
            }
            Element::Text {
                content,
                padding: p,
                width,
                ..
            } => {
                let fonts = self.fonts.as_mut().unwrap();
                let font = fonts.default();
                let (tw, th) = fonts.measure(content, font);
                let w = Self::resolve_size(width, tw, avail_w - p.left - p.right);
                (w + p.left + p.right, th + p.top + p.bottom)
            }
            Element::Button {
                w,
                h,
                padding: p,
                width,
                ..
            } => {
                let w = Self::resolve_size(width, *w, avail_w - p.left - p.right);
                (w + p.left + p.right, *h + p.top + p.bottom)
            }
            Element::Container {
                padding: p,
                width,
                child,
                ..
            } => {
                let inner_avail_w = avail_w - p.left - p.right;
                let (cw, ch) =
                    self.layout_measure(child, inner_avail_w, avail_h - p.top - p.bottom);
                let w = Self::resolve_size(width, cw + p.left + p.right, avail_w);
                (w, ch + p.top + p.bottom)
            }
            Element::Column {
                gap,
                padding: p,
                width,
                children,
                ..
            } => {
                let inner_avail_w = avail_w - p.left - p.right;
                let mut max_w = 0.0f32;
                let mut total_h = 0.0f32;
                for (i, child) in children.iter().enumerate() {
                    let (cw, ch) = self.layout_measure(child, inner_avail_w, avail_h);
                    max_w = max_w.max(cw);
                    total_h += ch;
                    if i < children.len() - 1 {
                        total_h += gap;
                    }
                }
                let w = Self::resolve_size(width, max_w + p.left + p.right, avail_w);
                (w, total_h + p.top + p.bottom)
            }
            Element::Empty => (0.0, 0.0),
            _ => (avail_w, avail_h),
        }
    }

    fn layout(
        &mut self,
        element: Element<A>,
        x: f32,
        y: f32,
        avail_w: f32,
        avail_h: f32,
    ) -> LayoutNode<A> {
        match element {
            Element::Rect {
                w,
                h,
                color,
                hover_color,
                padding: p,
                width,
                height,
                callbacks,
            } => {
                let w = Self::resolve_size(&width, w, avail_w - p.left - p.right);
                let h = Self::resolve_size(&height, h, avail_h - p.top - p.bottom);
                LayoutNode {
                    x: x + p.left,
                    y: y + p.top,
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
                width,
            } => {
                let fonts = self.fonts.as_mut().unwrap();
                let font = fonts.default();
                let (tw, th) = fonts.measure(&content, font);
                let w = Self::resolve_size(&width, tw, avail_w - p.left - p.right);
                LayoutNode {
                    x: x + p.left,
                    y: y + p.top,
                    w,
                    h: th,
                    kind: LayoutKind::Text { content, color },
                }
            }
            Element::Button {
                label,
                w,
                h,
                style,
                padding: p,
                width,
                on_click,
            } => {
                let w = Self::resolve_size(&width, w, avail_w - p.left - p.right);
                LayoutNode {
                    x: x + p.left,
                    y: y + p.top,
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
                width,
                children,
            } => {
                let inner_avail_w = Self::resolve_size(&width, avail_w, avail_w) - p.left - p.right;
                let inner_avail_h = avail_h - p.top - p.bottom;
                let n = children.len();

                // first pass is measure fixed children, count fill children
                let mut fixed_w = if n > 0 { gap * (n as f32 - 1.0) } else { 0.0 };
                let mut fill_count = 0usize;
                for child in &children {
                    if Self::child_is_fill_w(child) {
                        fill_count += 1;
                    } else {
                        let node = self.layout_measure(child, inner_avail_w, inner_avail_h);
                        fixed_w += node.0;
                    }
                }
                let fill_w = if fill_count > 0 {
                    (inner_avail_w - fixed_w).max(0.0) / fill_count as f32
                } else {
                    0.0
                };

                // second pass is layout with resolved widths
                let mut cursor_x = x + p.left;
                let mut max_h = 0.0f32;
                let mut nodes = Vec::with_capacity(n);
                for child in children {
                    let child_avail = if Self::child_is_fill_w(&child) {
                        fill_w
                    } else {
                        inner_avail_w
                    };
                    let node = self.layout(child, cursor_x, y + p.top, child_avail, inner_avail_h);
                    cursor_x += node.w + gap;
                    max_h = max_h.max(node.h);
                    nodes.push(node);
                }
                let w = Self::resolve_size(&width, cursor_x - x - gap + p.right, avail_w);
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
                width,
                children,
            } => {
                let inner_avail_w = Self::resolve_size(&width, avail_w, avail_w) - p.left - p.right;
                let n = children.len();

                let mut cursor_y = y + p.top;
                let mut max_w = 0.0f32;
                let mut nodes = Vec::with_capacity(n);
                for child in children {
                    let node = self.layout(child, x + p.left, cursor_y, inner_avail_w, avail_h);
                    cursor_y += node.h + gap;
                    max_w = max_w.max(node.w);
                    nodes.push(node);
                }
                let w = Self::resolve_size(&width, max_w + p.left + p.right, avail_w);
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
                width,
                height,
                child,
            } => {
                let inner_avail_w = Self::resolve_size(&width, avail_w, avail_w) - p.left - p.right;
                let inner = self.layout(*child, x + p.left, y + p.top, inner_avail_w, avail_h);
                let w = Self::resolve_size(&width, inner.w + p.left + p.right, avail_w);
                let h = match &height {
                    None => inner.h + p.top + p.bottom,
                    Some(Size::Fill) => avail_h,
                    Some(Size::Fixed(v)) => *v,
                    Some(Size::Percent(pct)) => avail_h * pct / 100.0,
                };
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
        let mut layout = self.layout(tree, 0.0, 0.0, width, height);

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

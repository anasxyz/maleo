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
    Align, Color, Element, Events, Font, FontId, Fonts, GpuContext, ShapeRenderer, Size,
    TextRenderer,
};

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
        layout(
            &mut tree,
            0.0,
            0.0,
            width,
            height,
            self.fonts.as_mut().unwrap(),
        );
        println!("Rendering...");
        self.draw_element(&mut tree);

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

    fn draw_element(&mut self, el: &mut Element) {
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
            Element::Button {
                label,
                style,
                resolved_w,
                resolved_h,
                on_click,
            } => {
                let hovered = self.events.mouse.over(style.x, style.y, *resolved_w, *resolved_h);
                let clicked = hovered && self.events.mouse.left_just_pressed;

                let bg = if clicked {
                    style.background.map(|c| Color::rgb(
                        (c.r + 0.15).min(1.0),
                        (c.g + 0.15).min(1.0),
                        (c.b + 0.15).min(1.0),
                    )).unwrap_or(Color::rgb(0.5, 0.5, 0.6))
                } else if hovered {
                    style.background.map(|c| Color::rgb(
                        (c.r + 0.08).min(1.0),
                        (c.g + 0.08).min(1.0),
                        (c.b + 0.08).min(1.0),
                    )).unwrap_or(Color::rgb(0.35, 0.35, 0.45))
                } else {
                    style.background.unwrap_or(Color::rgb(0.25, 0.25, 0.35))
                };

                self.shape_renderer.as_mut().unwrap().draw_rect(
                    style.x, style.y, *resolved_w, *resolved_h,
                    bg.to_array(), [0.0; 4], 0.0,
                );

                // draw label centered with fixed internal spacing
                let fonts = self.fonts.as_mut().unwrap();
                let font_id = fonts.default().unwrap();
                let entry = fonts.get(font_id);
                let family = entry.family.clone();
                let size = entry.size;
                let (tw, th) = fonts.measure(label, font_id);
                let tx = style.x + (*resolved_w - tw) / 2.0;
                let ty = style.y + (*resolved_h - th) / 2.0;
                self.text_renderer.as_mut().unwrap().draw(
                    &mut self.fonts.as_mut().unwrap().font_system,
                    family, size, label, tx, ty,
                    Color::rgb(0.92, 0.92, 0.95),
                );

                if clicked {
                    if let Some(cb) = on_click {
                        cb();
                    }
                }
            }
            Element::Row {
                style,
                children,
                resolved_w,
                resolved_h,
            } => {
                if let Some(bg) = style.background {
                    self.shape_renderer.as_mut().unwrap().draw_rect(
                        style.x,
                        style.y,
                        *resolved_w,
                        *resolved_h,
                        bg.to_array(),
                        [0.0; 4],
                        0.0,
                    );
                }
                for child in children {
                    self.draw_element(child);
                }
            }
            Element::Column {
                style,
                children,
                resolved_w,
                resolved_h,
            } => {
                if let Some(bg) = style.background {
                    self.shape_renderer.as_mut().unwrap().draw_rect(
                        style.x,
                        style.y,
                        *resolved_w,
                        *resolved_h,
                        bg.to_array(),
                        [0.0; 4],
                        0.0,
                    );
                }
                for child in children {
                    self.draw_element(child);
                }
            }
        }
    }
}

fn child_width_sizing(element: &Element) -> Size {
    match element {
        Element::Empty => Size::Fixed(0.0),
        Element::Rect { style, w, .. } => style.width.clone().unwrap_or(Size::Fixed(*w)),
        Element::Text { style, .. } => style.width.clone().unwrap_or(Size::Fixed(0.0)),
        Element::Row { style, .. } => style.width.clone().unwrap_or(Size::Fill),
        Element::Column { style, .. } => style.width.clone().unwrap_or(Size::Fill),
        Element::Button { style, .. } => style.width.clone().unwrap_or(Size::Fixed(0.0)),
    }
}

fn child_height_sizing(element: &Element) -> Size {
    match element {
        Element::Empty => Size::Fixed(0.0),
        Element::Rect { style, h, .. } => style.height.clone().unwrap_or(Size::Fixed(*h)),
        Element::Text { style, .. } => style.height.clone().unwrap_or(Size::Fixed(0.0)),
        Element::Row { style, .. } => style.height.clone().unwrap_or(Size::Fixed(0.0)),
        Element::Column { style, .. } => style.height.clone().unwrap_or(Size::Fixed(0.0)),
        Element::Button { style, .. } => style.height.clone().unwrap_or(Size::Fixed(0.0)),
    }
}

fn measure(element: &Element, fonts: &mut Fonts) -> (f32, f32) {
    match element {
        Element::Empty => (0.0, 0.0),
        Element::Rect { w, h, style, .. } => {
            let mw = match style.width.as_ref().unwrap_or(&Size::Fixed(*w)) {
                Size::Fill | Size::Percent(_) => 0.0,
                Size::Fixed(v) => *v,
            };
            let mh = match style.height.as_ref().unwrap_or(&Size::Fixed(*h)) {
                Size::Fill | Size::Percent(_) => 0.0,
                Size::Fixed(v) => *v,
            };
            (
                mw + style.padding.left + style.padding.right,
                mh + style.padding.top + style.padding.bottom,
            )
        }
        Element::Text {
            content,
            font,
            style,
            ..
        } => {
            let font_id = match font {
                Font::Name(name) => fonts.get_by_name(name).or_else(|| fonts.default()),
                Font::Default => fonts.default(),
            };
            let (w, h) = fonts.measure(content, font_id.unwrap());
            let mw = match style.width.as_ref() {
                Some(Size::Fill) | Some(Size::Percent(_)) => 0.0,
                Some(Size::Fixed(v)) => *v,
                None => w,
            };
            let mh = match style.height.as_ref() {
                Some(Size::Fill) | Some(Size::Percent(_)) => 0.0,
                Some(Size::Fixed(v)) => *v,
                None => h,
            };
            (
                mw + style.padding.left + style.padding.right,
                mh + style.padding.top + style.padding.bottom,
            )
        }
        Element::Row {
            style, children, ..
        } => {
            let mut total_width: f32 = 0.0;
            let mut max_height: f32 = 0.0;
            for child in children {
                let (cw, ch) = measure(child, fonts);
                total_width += cw;
                max_height = max_height.max(ch);
            }
            let gap_total = if children.is_empty() {
                0.0
            } else {
                style.gap * (children.len() - 1) as f32
            };
            (
                total_width + gap_total + style.padding.left + style.padding.right,
                max_height + style.padding.top + style.padding.bottom,
            )
        }
        Element::Column {
            style, children, ..
        } => {
            let mut max_width: f32 = 0.0;
            let mut total_height: f32 = 0.0;
            for child in children {
                let (cw, ch) = measure(child, fonts);
                max_width = max_width.max(cw);
                total_height += ch;
            }
            let gap_total = if children.is_empty() {
                0.0
            } else {
                style.gap * (children.len() - 1) as f32
            };
            (
                max_width + style.padding.left + style.padding.right,
                total_height + gap_total + style.padding.top + style.padding.bottom,
            )
        }
        Element::Button { label, style, .. } => {
            let font_id = fonts.default().unwrap();
            let (tw, th) = fonts.measure(label, font_id);
            let mw = match style.width.as_ref() {
                Some(Size::Fill) | Some(Size::Percent(_)) => 0.0,
                Some(Size::Fixed(v)) => *v,
                None => tw + 24.0,
            };
            let mh = match style.height.as_ref() {
                Some(Size::Fill) | Some(Size::Percent(_)) => 0.0,
                Some(Size::Fixed(v)) => *v,
                None => th + 12.0,
            };
            (mw + style.padding.left + style.padding.right, mh + style.padding.top + style.padding.bottom)
        }
    }
}

fn layout(
    element: &mut Element,
    x: f32,
    y: f32,
    available_w: f32,
    available_h: f32,
    fonts: &mut Fonts,
) {
    match element {
        Element::Empty => {}
        Element::Rect { style, w, h, .. } => {
            style.x = x + style.padding.left;
            style.y = y + style.padding.top;
            *w = available_w - style.padding.left - style.padding.right;
            *h = available_h - style.padding.top - style.padding.bottom;
        }
        Element::Text { style, .. } => {
            style.x = x + style.padding.left;
            style.y = y + style.padding.top;
        }
        Element::Button { style, resolved_w, resolved_h, .. } => {
            style.x = x + style.padding.left;
            style.y = y + style.padding.top;
            *resolved_w = match style.width.as_ref() {
                Some(Size::Fixed(v)) => *v,
                Some(Size::Fill) | None => available_w - style.padding.left - style.padding.right,
                Some(Size::Percent(p)) => (available_w - style.padding.left - style.padding.right) * p / 100.0,
            };
            *resolved_h = match style.height.as_ref() {
                Some(Size::Fixed(v)) => *v,
                Some(Size::Fill) | None => available_h - style.padding.top - style.padding.bottom,
                Some(Size::Percent(p)) => (available_h - style.padding.top - style.padding.bottom) * p / 100.0,
            };
        }
        Element::Row {
            style,
            children,
            resolved_w,
            resolved_h,
        } => {
            style.x = x;
            style.y = y;

            let self_w = available_w;
            let self_h = available_h;

            *resolved_w = self_w;
            *resolved_h = self_h;

            let inner_w = self_w - style.padding.left - style.padding.right;
            let inner_h = self_h - style.padding.top - style.padding.bottom;
            let gap = style.gap;
            let last = children.len().saturating_sub(1);
            let gap_total = gap * last as f32;

            let mut fixed_width: f32 = 0.0;
            let mut fill_count: u32 = 0;
            for child in children.iter() {
                match child_width_sizing(child) {
                    Size::Fill => fill_count += 1,
                    Size::Percent(p) => fixed_width += (inner_w - gap_total) * p / 100.0,
                    Size::Fixed(_) => fixed_width += measure(child, fonts).0,
                }
            }

            let remaining = (inner_w - fixed_width - gap_total).max(0.0);
            let fill_width = if fill_count > 0 {
                remaining / fill_count as f32
            } else {
                0.0
            };

            let total_children_w: f32 = children
                .iter()
                .enumerate()
                .map(|(i, child)| {
                    let cw = match child_width_sizing(child) {
                        Size::Fill => fill_width,
                        Size::Percent(p) => (inner_w - gap_total) * p / 100.0,
                        Size::Fixed(_) => measure(child, fonts).0,
                    };
                    cw + if i < last { gap } else { 0.0 }
                })
                .sum();
            let x_offset = match style.align_x {
                Align::Start => 0.0,
                Align::Center => (inner_w - total_children_w) / 2.0,
                Align::End => inner_w - total_children_w,
            };

            let mut cursor_x = x + style.padding.left + x_offset;
            let base_y = y + style.padding.top;
            for (i, child) in children.iter_mut().enumerate() {
                let child_w = match child_width_sizing(child) {
                    Size::Fill => fill_width,
                    Size::Percent(p) => (inner_w - gap_total) * p / 100.0,
                    Size::Fixed(_) => measure(child, fonts).0,
                };
                let child_h = match child_height_sizing(child) {
                    Size::Fill => inner_h,
                    Size::Percent(p) => inner_h * p / 100.0,
                    Size::Fixed(_) => measure(child, fonts).1,
                };
                let child_y = match style.align_y {
                    Align::Start => base_y,
                    Align::Center => base_y + (inner_h - child_h) / 2.0,
                    Align::End => base_y + inner_h - child_h,
                };
                layout(child, cursor_x, child_y, child_w, child_h, fonts);
                cursor_x += child_w + if i < last { gap } else { 0.0 };
            }
        }
        Element::Column {
            style,
            children,
            resolved_w,
            resolved_h,
        } => {
            style.x = x;
            style.y = y;

            let self_w = available_w;
            let self_h = available_h;

            *resolved_w = self_w;
            *resolved_h = self_h;

            let inner_w = self_w - style.padding.left - style.padding.right;
            let inner_h = self_h - style.padding.top - style.padding.bottom;
            let gap = style.gap;
            let last = children.len().saturating_sub(1);
            let gap_total = gap * last as f32;

            let mut fixed_height: f32 = 0.0;
            let mut fill_count: u32 = 0;
            for child in children.iter() {
                match child_height_sizing(child) {
                    Size::Fill => fill_count += 1,
                    Size::Percent(p) => fixed_height += (inner_h - gap_total) * p / 100.0,
                    Size::Fixed(_) => fixed_height += measure(child, fonts).1,
                }
            }

            let remaining = (inner_h - fixed_height - gap_total).max(0.0);
            let fill_height = if fill_count > 0 {
                remaining / fill_count as f32
            } else {
                0.0
            };

            let total_children_h: f32 = children
                .iter()
                .enumerate()
                .map(|(i, child)| {
                    let ch = match child_height_sizing(child) {
                        Size::Fill => fill_height,
                        Size::Percent(p) => (inner_h - gap_total) * p / 100.0,
                        Size::Fixed(_) => measure(child, fonts).1,
                    };
                    ch + if i < last { gap } else { 0.0 }
                })
                .sum();
            let y_offset = match style.align_y {
                Align::Start => 0.0,
                Align::Center => (inner_h - total_children_h) / 2.0,
                Align::End => inner_h - total_children_h,
            };

            let base_x = x + style.padding.left;
            let mut cursor_y = y + style.padding.top + y_offset;
            for (i, child) in children.iter_mut().enumerate() {
                let child_w = match child_width_sizing(child) {
                    Size::Fill => inner_w,
                    Size::Percent(p) => inner_w * p / 100.0,
                    Size::Fixed(_) => measure(child, fonts).0,
                };
                let child_h = match child_height_sizing(child) {
                    Size::Fill => fill_height,
                    Size::Percent(p) => (inner_h - gap_total) * p / 100.0,
                    Size::Fixed(_) => measure(child, fonts).1,
                };
                let child_x = match style.align_x {
                    Align::Start => base_x,
                    Align::Center => base_x + (inner_w - child_w) / 2.0,
                    Align::End => base_x + inner_w - child_w,
                };
                layout(child, child_x, cursor_y, child_w, child_h, fonts);
                cursor_y += child_h + if i < last { gap } else { 0.0 };
            }
        }
    }
}

fn measure_children_width(children: &[Element], style: &crate::Style, fonts: &mut Fonts) -> f32 {
    let mut total = 0.0f32;
    for child in children {
        total += measure(child, fonts).0;
    }
    let gap_total = if children.is_empty() {
        0.0
    } else {
        style.gap * (children.len() - 1) as f32
    };
    total + gap_total + style.padding.left + style.padding.right
}

fn measure_children_height(children: &[Element], style: &crate::Style, fonts: &mut Fonts) -> f32 {
    let mut total = 0.0f32;
    for child in children {
        total += measure(child, fonts).1;
    }
    let gap_total = if children.is_empty() {
        0.0
    } else {
        style.gap * (children.len() - 1) as f32
    };
    total + gap_total + style.padding.top + style.padding.bottom
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

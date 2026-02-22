use std::sync::Arc;

use taffy::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{
    Align, Color, Element, Events, Font, Fonts, GpuContext, Overflow, Position, ShapeRenderer,
    TextRenderer, Val,
};

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

// app trait

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
    fonts: Option<Fonts>,
    events: Events,
    clear_color: Color,
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
        do_layout(&mut tree, width, height, self.fonts.as_mut().unwrap());
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
        self.draw_clipped(el, None);
    }

    fn draw_clipped(&mut self, el: &mut Element, clip: Option<[f32; 4]>) {
        match el {
            Element::Empty => {}

            Element::Rect {
                color,
                style,
                resolved_w,
                resolved_h,
            } => {
                if is_outside(style.x, style.y, *resolved_w, *resolved_h, clip) {
                    return;
                }
                draw_shape(
                    self.shape_renderer.as_mut().unwrap(),
                    style.x,
                    style.y,
                    *resolved_w,
                    *resolved_h,
                    color.to_array(),
                    style.border_radius,
                );
            }

            Element::Text {
                content,
                color,
                font,
                style,
            } => {
                if is_outside(style.x, style.y, 1.0, 1.0, clip) {
                    return;
                }
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
                on_click,
                resolved_x,
                resolved_y,
                resolved_w,
                resolved_h,
            } => {
                if is_outside(*resolved_x, *resolved_y, *resolved_w, *resolved_h, clip) {
                    return;
                }
                let hovered =
                    self.events
                        .mouse
                        .over(*resolved_x, *resolved_y, *resolved_w, *resolved_h);
                let clicked = hovered && self.events.mouse.left_just_pressed;

                let bg = if clicked {
                    style
                        .background
                        .map(|c| {
                            Color::rgb(
                                (c.r + 0.15).min(1.0),
                                (c.g + 0.15).min(1.0),
                                (c.b + 0.15).min(1.0),
                            )
                        })
                        .unwrap_or(Color::rgb(0.5, 0.5, 0.6))
                } else if hovered {
                    style
                        .background
                        .map(|c| {
                            Color::rgb(
                                (c.r + 0.08).min(1.0),
                                (c.g + 0.08).min(1.0),
                                (c.b + 0.08).min(1.0),
                            )
                        })
                        .unwrap_or(Color::rgb(0.35, 0.35, 0.45))
                } else {
                    style.background.unwrap_or(Color::rgb(0.25, 0.25, 0.35))
                };

                draw_shape(
                    self.shape_renderer.as_mut().unwrap(),
                    *resolved_x,
                    *resolved_y,
                    *resolved_w,
                    *resolved_h,
                    bg.to_array(),
                    style.border_radius,
                );

                let fonts = self.fonts.as_mut().unwrap();
                let font_id = fonts.default().unwrap();
                let entry = fonts.get(font_id);
                let family = entry.family.clone();
                let size = entry.size;
                let (tw, th) = fonts.measure(label, font_id);
                let tx = *resolved_x + (*resolved_w - tw) / 2.0;
                let ty = *resolved_y + (*resolved_h - th) / 2.0;
                self.text_renderer.as_mut().unwrap().draw(
                    &mut self.fonts.as_mut().unwrap().font_system,
                    family,
                    size,
                    label,
                    tx,
                    ty,
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
                    draw_shape(
                        self.shape_renderer.as_mut().unwrap(),
                        style.x,
                        style.y,
                        *resolved_w,
                        *resolved_h,
                        bg.to_array(),
                        style.border_radius,
                    );
                }
                let child_clip = child_clip(
                    style.x,
                    style.y,
                    *resolved_w,
                    *resolved_h,
                    style.overflow,
                    clip,
                );
                for child in children {
                    self.draw_clipped(child, child_clip);
                }
            }

            Element::Column {
                style,
                children,
                resolved_w,
                resolved_h,
            } => {
                if let Some(bg) = style.background {
                    draw_shape(
                        self.shape_renderer.as_mut().unwrap(),
                        style.x,
                        style.y,
                        *resolved_w,
                        *resolved_h,
                        bg.to_array(),
                        style.border_radius,
                    );
                }
                let child_clip = child_clip(
                    style.x,
                    style.y,
                    *resolved_w,
                    *resolved_h,
                    style.overflow,
                    clip,
                );
                for child in children {
                    self.draw_clipped(child, child_clip);
                }
            }
        }
    }
}

// clipping helpers

// returns true if the element is fully outside the clip rect
fn is_outside(x: f32, y: f32, w: f32, h: f32, clip: Option<[f32; 4]>) -> bool {
    let Some([cx, cy, cx2, cy2]) = clip else {
        return false;
    };
    x + w < cx || y + h < cy || x > cx2 || y > cy2
}

// computes the clip rect to pass to children
fn child_clip(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    overflow: Overflow,
    parent_clip: Option<[f32; 4]>,
) -> Option<[f32; 4]> {
    if overflow == Overflow::Hidden || overflow == Overflow::Scroll {
        let new_clip = [x, y, x + w, y + h];
        // intersect with parent clip if there is one
        if let Some([px, py, px2, py2]) = parent_clip {
            Some([
                new_clip[0].max(px),
                new_clip[1].max(py),
                new_clip[2].min(px2),
                new_clip[3].min(py2),
            ])
        } else {
            Some(new_clip)
        }
    } else {
        parent_clip
    }
}

// taffy helpers

// draw a rect or rounded rect depending on border_radius
fn draw_shape(
    sr: &mut crate::ShapeRenderer,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 4],
    border_radius: f32,
) {
    if border_radius > 0.0 {
        sr.draw_rounded_rect(x, y, w, h, border_radius, color, [0.0; 4], 0.0);
    } else {
        sr.draw_rect(x, y, w, h, color, [0.0; 4], 0.0);
    }
}

fn val_to_dimension(v: &Val) -> Dimension {
    match v {
        Val::Auto => Dimension::Auto,
        Val::Px(v) => Dimension::Length(*v),
        Val::Percent(p) => Dimension::Percent(*p / 100.0),
    }
}

fn val_to_lpa(v: &Val) -> LengthPercentageAuto {
    match v {
        Val::Auto => LengthPercentageAuto::Auto,
        Val::Px(v) => LengthPercentageAuto::Length(*v),
        Val::Percent(p) => LengthPercentageAuto::Percent(*p / 100.0),
    }
}

fn edges_to_rect_lp(e: &crate::Edges) -> Rect<LengthPercentage> {
    Rect {
        left: LengthPercentage::Length(e.left),
        right: LengthPercentage::Length(e.right),
        top: LengthPercentage::Length(e.top),
        bottom: LengthPercentage::Length(e.bottom),
    }
}

fn edges_to_rect_lpa(e: &crate::Edges) -> Rect<LengthPercentageAuto> {
    Rect {
        left: LengthPercentageAuto::Length(e.left),
        right: LengthPercentageAuto::Length(e.right),
        top: LengthPercentageAuto::Length(e.top),
        bottom: LengthPercentageAuto::Length(e.bottom),
    }
}

fn align_to_justify(a: Align) -> Option<JustifyContent> {
    Some(match a {
        Align::Start => JustifyContent::FlexStart,
        Align::Center => JustifyContent::Center,
        Align::End => JustifyContent::FlexEnd,
        Align::SpaceBetween => JustifyContent::SpaceBetween,
        Align::SpaceAround => JustifyContent::SpaceAround,
        Align::SpaceEvenly => JustifyContent::SpaceEvenly,
    })
}

fn align_to_items(a: Align) -> Option<AlignItems> {
    Some(match a {
        Align::Start => AlignItems::FlexStart,
        Align::Center => AlignItems::Center,
        Align::End => AlignItems::FlexEnd,
        _ => AlignItems::Stretch,
    })
}

fn align_to_self(a: Align) -> Option<AlignSelf> {
    match a {
        Align::Start => Some(AlignSelf::FlexStart),
        Align::Center => Some(AlignSelf::Center),
        Align::End => Some(AlignSelf::FlexEnd),
        _ => None,
    }
}

fn overflow_to_taffy(o: Overflow) -> taffy::geometry::Point<taffy::style::Overflow> {
    let v = match o {
        Overflow::Visible => taffy::style::Overflow::Visible,
        Overflow::Hidden => taffy::style::Overflow::Hidden,
        Overflow::Scroll => taffy::style::Overflow::Scroll,
    };
    taffy::geometry::Point { x: v, y: v }
}

fn style_to_taffy(style: &crate::Style, flex_direction: FlexDirection) -> taffy::Style {
    taffy::Style {
        display: Display::Flex,
        flex_direction,
        flex_wrap: if style.wrap {
            FlexWrap::Wrap
        } else {
            FlexWrap::NoWrap
        },
        position: match style.position {
            Position::Relative => taffy::style::Position::Relative,
            Position::Absolute => taffy::style::Position::Absolute,
        },
        inset: Rect {
            left: LengthPercentageAuto::Length(style.inset.left),
            right: LengthPercentageAuto::Length(style.inset.right),
            top: LengthPercentageAuto::Length(style.inset.top),
            bottom: LengthPercentageAuto::Length(style.inset.bottom),
        },
        size: taffy::geometry::Size {
            width: val_to_dimension(&style.width),
            height: val_to_dimension(&style.height),
        },
        min_size: taffy::geometry::Size {
            width: val_to_dimension(&style.min_width),
            height: val_to_dimension(&style.min_height),
        },
        max_size: taffy::geometry::Size {
            width: val_to_dimension(&style.max_width),
            height: val_to_dimension(&style.max_height),
        },
        aspect_ratio: style.aspect_ratio,
        flex_grow: style.grow,
        flex_shrink: style.shrink.unwrap_or(1.0),
        flex_basis: val_to_dimension(&style.basis),
        padding: edges_to_rect_lp(&style.padding),
        margin: edges_to_rect_lpa(&style.margin),
        gap: taffy::geometry::Size {
            width: LengthPercentage::Length(style.gap),
            height: LengthPercentage::Length(style.gap),
        },
        align_self: style.align_self.and_then(align_to_self),
        overflow: overflow_to_taffy(style.overflow),
        ..Default::default()
    }
}

// build taffy node tree

fn build_taffy_node(taffy: &mut TaffyTree<()>, element: &Element, fonts: &mut Fonts) -> NodeId {
    match element {
        Element::Empty => taffy.new_leaf(taffy::Style::default()).unwrap(),

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
            taffy
                .new_leaf(taffy::Style {
                    size: taffy::geometry::Size {
                        width: Dimension::Length(w),
                        height: Dimension::Length(h),
                    },
                    margin: edges_to_rect_lpa(&style.margin),
                    flex_grow: style.grow,
                    flex_shrink: 1.0,
                    align_self: style.align_self.and_then(align_to_self),
                    ..Default::default()
                })
                .unwrap()
        }

        Element::Rect { style, .. } => {
            let mut ts = style_to_taffy(style, FlexDirection::Row);
            ts.justify_content = None;
            ts.align_items = None;
            taffy.new_leaf(ts).unwrap()
        }

        Element::Button { label, style, .. } => {
            let font_id = fonts.default().unwrap();
            let (tw, th) = fonts.measure(label, font_id);
            let natural_w = tw + 24.0;
            let natural_h = th + 12.0;
            taffy
                .new_leaf(taffy::Style {
                    size: taffy::geometry::Size {
                        width: match &style.width {
                            Val::Auto => Dimension::Length(natural_w),
                            other => val_to_dimension(other),
                        },
                        height: match &style.height {
                            Val::Auto => Dimension::Length(natural_h),
                            other => val_to_dimension(other),
                        },
                    },
                    margin: edges_to_rect_lpa(&style.margin),
                    flex_grow: style.grow,
                    flex_shrink: 1.0,
                    align_self: style.align_self.and_then(align_to_self),
                    ..Default::default()
                })
                .unwrap()
        }

        Element::Row {
            style, children, ..
        } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| build_taffy_node(taffy, c, fonts))
                .collect();
            let mut ts = style_to_taffy(style, FlexDirection::Row);
            ts.justify_content = align_to_justify(style.align_x);
            ts.align_items = align_to_items(style.align_y);
            taffy.new_with_children(ts, &child_nodes).unwrap()
        }

        Element::Column {
            style, children, ..
        } => {
            let child_nodes: Vec<NodeId> = children
                .iter()
                .map(|c| build_taffy_node(taffy, c, fonts))
                .collect();
            let mut ts = style_to_taffy(style, FlexDirection::Column);
            ts.justify_content = align_to_justify(style.align_y);
            ts.align_items = align_to_items(style.align_x);
            taffy.new_with_children(ts, &child_nodes).unwrap()
        }
    }
}

// apply computed layout back onto elements

fn apply_layout(
    taffy: &TaffyTree<()>,
    element: &mut Element,
    node: NodeId,
    parent_x: f32,
    parent_y: f32,
) {
    let layout = taffy.layout(node).unwrap();
    let x = parent_x + layout.location.x;
    let y = parent_y + layout.location.y;
    let w = layout.size.width;
    let h = layout.size.height;

    match element {
        Element::Empty => {}
        Element::Text { style, .. } => {
            style.x = x;
            style.y = y;
        }
        Element::Rect {
            style,
            resolved_w,
            resolved_h,
            ..
        } => {
            style.x = x;
            style.y = y;
            *resolved_w = w;
            *resolved_h = h;
        }
        Element::Button {
            resolved_x,
            resolved_y,
            resolved_w,
            resolved_h,
            ..
        } => {
            *resolved_x = x;
            *resolved_y = y;
            *resolved_w = w;
            *resolved_h = h;
        }
        Element::Row {
            style,
            children,
            resolved_w,
            resolved_h,
        } => {
            style.x = x;
            style.y = y;
            *resolved_w = w;
            *resolved_h = h;
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
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
            *resolved_w = w;
            *resolved_h = h;
            let child_nodes = taffy.children(node).unwrap();
            for (child, child_node) in children.iter_mut().zip(child_nodes.iter()) {
                apply_layout(taffy, child, *child_node, x, y);
            }
        }
    }
}

fn do_layout(element: &mut Element, width: f32, height: f32, fonts: &mut Fonts) {
    let mut taffy: TaffyTree<()> = TaffyTree::new();
    let root = build_taffy_node(&mut taffy, element, fonts);
    taffy
        .compute_layout(
            root,
            taffy::geometry::Size {
                width: AvailableSpace::Definite(width),
                height: AvailableSpace::Definite(height),
            },
        )
        .unwrap();
    apply_layout(&taffy, element, root, 0.0, 0.0);
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
            self.text_renderer = Some(text_renderer);
            self.shape_renderer = Some(shape_renderer);
        }

        let mut fonts = Fonts::new();
        self.app.fonts(&mut fonts);
        if fonts.default().is_none() {
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

use crate::Color;

pub struct ButtonStyle {
    pub color: Color,
    pub hover_color: Color,
    pub text_color: Color,
    pub corner_radius: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            color: Color::rgb(0.2, 0.2, 0.8),
            hover_color: Color::rgb(0.3, 0.3, 1.0),
            text_color: Color::WHITE,
            corner_radius: 4.0,
        }
    }
}

pub struct Callbacks<A> {
    pub on_click: Option<Box<dyn FnMut(&mut A)>>,
    pub on_hover: Option<Box<dyn FnMut(&mut A)>>,
    pub on_just_hovered: Option<Box<dyn FnMut(&mut A)>>,
    pub on_just_unhovered: Option<Box<dyn FnMut(&mut A)>>,
}

impl<A> Callbacks<A> {
    fn none() -> Self {
        Self { on_click: None, on_hover: None, on_just_hovered: None, on_just_unhovered: None }
    }
}

pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
    pub fn all(v: f32) -> Self { Self { top: v, right: v, bottom: v, left: v } }
    pub fn horizontal(v: f32) -> Self { Self { top: 0.0, right: v, bottom: 0.0, left: v } }
    pub fn vertical(v: f32) -> Self { Self { top: v, right: 0.0, bottom: v, left: 0.0 } }
    pub fn top(v: f32) -> Self { Self { top: v, right: 0.0, bottom: 0.0, left: 0.0 } }
    pub fn bottom(v: f32) -> Self { Self { top: 0.0, right: 0.0, bottom: v, left: 0.0 } }
    pub fn left(v: f32) -> Self { Self { top: 0.0, right: 0.0, bottom: 0.0, left: v } }
    pub fn right(v: f32) -> Self { Self { top: 0.0, right: v, bottom: 0.0, left: 0.0 } }
}

impl Default for Padding {
    fn default() -> Self { Self::all(0.0) }
}

pub enum Element<A> {
    Rect {
        w: f32,
        h: f32,
        color: Color,
        hover_color: Option<Color>,
        padding: Padding,
        callbacks: Callbacks<A>,
    },
    Text {
        content: String,
        color: Color,
        padding: Padding,
    },
    Button {
        label: String,
        w: f32,
        h: f32,
        style: ButtonStyle,
        padding: Padding,
        on_click: Option<Box<dyn FnMut(&mut A)>>,
    },
    Row {
        gap: f32,
        padding: Padding,
        children: Vec<Element<A>>,
    },
    Column {
        gap: f32,
        padding: Padding,
        children: Vec<Element<A>>,
    },
    Empty,
}

impl<A: 'static> Element<A> {
    pub fn gap(mut self, gap: f32) -> Self {
        match &mut self {
            Element::Row { gap: g, .. } => *g = gap,
            Element::Column { gap: g, .. } => *g = gap,
            _ => {}
        }
        self
    }

    pub fn padding(mut self, p: Padding) -> Self {
        match &mut self {
            Element::Rect { padding, .. } => *padding = p,
            Element::Text { padding, .. } => *padding = p,
            Element::Button { padding, .. } => *padding = p,
            Element::Row { padding, .. } => *padding = p,
            Element::Column { padding, .. } => *padding = p,
            _ => {}
        }
        self
    }

    pub fn hover_color(mut self, color: Color) -> Self {
        if let Element::Rect { hover_color, .. } = &mut self {
            *hover_color = Some(color);
        }
        self
    }

    pub fn on_click(mut self, f: impl FnMut(&mut A) + 'static) -> Self {
        match &mut self {
            Element::Rect { callbacks, .. } => callbacks.on_click = Some(Box::new(f)),
            Element::Button { on_click, .. } => *on_click = Some(Box::new(f)),
            _ => {}
        }
        self
    }

    pub fn on_hover(mut self, f: impl FnMut(&mut A) + 'static) -> Self {
        if let Element::Rect { callbacks, .. } = &mut self {
            callbacks.on_hover = Some(Box::new(f));
        }
        self
    }

    pub fn just_hovered(mut self, f: impl FnMut(&mut A) + 'static) -> Self {
        if let Element::Rect { callbacks, .. } = &mut self {
            callbacks.on_just_hovered = Some(Box::new(f));
        }
        self
    }

    pub fn just_unhovered(mut self, f: impl FnMut(&mut A) + 'static) -> Self {
        if let Element::Rect { callbacks, .. } = &mut self {
            callbacks.on_just_unhovered = Some(Box::new(f));
        }
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        if let Element::Button { style: s, .. } = &mut self {
            *s = style;
        }
        self
    }
}

pub fn empty<A>() -> Element<A> { Element::Empty }

pub fn rect<A>(w: f32, h: f32, color: Color) -> Element<A> {
    Element::Rect { w, h, color, hover_color: None, padding: Padding::default(), callbacks: Callbacks::none() }
}

pub fn text<A>(content: &str, color: Color) -> Element<A> {
    Element::Text { content: content.to_string(), color, padding: Padding::default() }
}

pub fn button<A>(label: &str, w: f32, h: f32, color: Color) -> Element<A> {
    let hover_color = Color::rgb(
        (color.r + 0.1).min(1.0),
        (color.g + 0.1).min(1.0),
        (color.b + 0.1).min(1.0),
    );
    Element::Button {
        label: label.to_string(),
        w,
        h,
        style: ButtonStyle { color, hover_color, ..ButtonStyle::default() },
        padding: Padding::default(),
        on_click: None,
    }
}

pub fn row<A>(children: Vec<Element<A>>) -> Element<A> {
    Element::Row { gap: 0.0, padding: Padding::default(), children }
}

pub fn column<A>(children: Vec<Element<A>>) -> Element<A> {
    Element::Column { gap: 0.0, padding: Padding::default(), children }
}

pub struct LayoutNode<A> {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub kind: LayoutKind<A>,
}

pub enum LayoutKind<A> {
    Rect { color: Color, hover_color: Option<Color>, hovered: bool, callbacks: Callbacks<A> },
    Text { content: String, color: Color },
    Button { label: String, style: ButtonStyle, on_click: Option<Box<dyn FnMut(&mut A)>>, hovered: bool },
    Children(Vec<LayoutNode<A>>),
    Empty,
}

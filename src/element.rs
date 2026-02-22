use crate::{Color, Font};

// alignment

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

// val

#[derive(Clone, Default)]
pub enum Val {
    #[default]
    Auto,
    Px(f32),
    Percent(f32),
}

// edges, used for padding, margin, inset, etc

#[derive(Clone, Copy, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
    pub fn horizontal(v: f32) -> Self {
        Self {
            top: 0.0,
            right: v,
            bottom: 0.0,
            left: v,
        }
    }
    pub fn vertical(v: f32) -> Self {
        Self {
            top: v,
            right: 0.0,
            bottom: v,
            left: 0.0,
        }
    }
    pub fn top(v: f32) -> Self {
        Self {
            top: v,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }
    }
    pub fn bottom(v: f32) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: v,
            left: 0.0,
        }
    }
    pub fn left(v: f32) -> Self {
        Self {
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            left: v,
        }
    }
    pub fn right(v: f32) -> Self {
        Self {
            top: 0.0,
            right: v,
            bottom: 0.0,
            left: 0.0,
        }
    }
}

// keep padding as an alias for backwards compat and ergonomics
pub type Padding = Edges;

// position

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
}

// overflow

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
}

// style

#[derive(Clone)]
pub struct Style {
    // resolved position
    // this is set by layout, not by user
    pub x: f32,
    pub y: f32,

    // sizing
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub max_width: Val,
    pub min_height: Val,
    pub max_height: Val,
    pub aspect_ratio: Option<f32>,

    // flex
    pub grow: f32,
    pub shrink: Option<f32>,
    pub basis: Val,
    pub wrap: bool,

    // alignment (on containers)
    pub align_x: Align,
    pub align_y: Align,

    // alignment (
    // this is on self, overrides parentss align_items
    pub align_self: Option<Align>,

    // spacing
    pub padding: Edges,
    pub margin: Edges,
    pub gap: f32,

    // position
    pub position: Position,
    pub inset: Edges,

    // visuals
    pub background: Option<Color>,
    pub border_radius: f32,
    pub border_color: Option<Color>,
    pub border_thickness: f32,
    pub opacity: f32,
    pub overflow: Overflow,
    pub shadow_color: Color,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: Val::Auto,
            height: Val::Auto,
            min_width: Val::Auto,
            max_width: Val::Auto,
            min_height: Val::Auto,
            max_height: Val::Auto,
            aspect_ratio: None,
            grow: 0.0,
            shrink: None,
            basis: Val::Auto,
            wrap: false,
            align_x: Align::Start,
            align_y: Align::Start,
            align_self: None,
            padding: Edges::default(),
            margin: Edges::default(),
            gap: 0.0,
            position: Position::Relative,
            inset: Edges::default(),
            background: None,
            border_radius: 0.0,
            border_color: None,
            border_thickness: 0.0,
            opacity: 1.0,
            overflow: Overflow::Visible,
            shadow_color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            shadow_blur: 0.0,
        }
    }
}

// element

pub enum Element {
    Empty,
    Rect {
        color: Color,
        style: Style,
        // resolved by layout
        resolved_w: f32,
        resolved_h: f32,
    },
    Text {
        content: String,
        color: Color,
        font: Font,
        style: Style,
    },
    Button {
        label: String,
        style: Style,
        on_click: Option<Box<dyn FnMut()>>,
        // resolved by layout
        resolved_x: f32,
        resolved_y: f32,
        resolved_w: f32,
        resolved_h: f32,
    },
    Row {
        style: Style,
        children: Vec<Element>,
        // resolved by layout
        resolved_w: f32,
        resolved_h: f32,
    },
    Column {
        style: Style,
        children: Vec<Element>,
        // resolved by layout
        resolved_w: f32,
        resolved_h: f32,
    },
}

// builder impl

impl Element {
    fn style_mut(&mut self) -> Option<&mut Style> {
        match self {
            Element::Rect { style, .. } => Some(style),
            Element::Text { style, .. } => Some(style),
            Element::Row { style, .. } => Some(style),
            Element::Column { style, .. } => Some(style),
            Element::Button { style, .. } => Some(style),
            Element::Empty => None,
        }
    }

    // sizing
    pub fn width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.width = v;
        }
        self
    }
    pub fn height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.height = v;
        }
        self
    }
    pub fn min_width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.min_width = v;
        }
        self
    }
    pub fn max_width(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.max_width = v;
        }
        self
    }
    pub fn min_height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.min_height = v;
        }
        self
    }
    pub fn max_height(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.max_height = v;
        }
        self
    }
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.aspect_ratio = Some(ratio);
        }
        self
    }

    // flex
    pub fn grow(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.grow = v;
        }
        self
    }
    pub fn shrink(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.shrink = Some(v);
        }
        self
    }
    pub fn basis(mut self, v: Val) -> Self {
        if let Some(s) = self.style_mut() {
            s.basis = v;
        }
        self
    }
    pub fn wrap(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.wrap = true;
        }
        self
    }

    // alignment
    pub fn align_x(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_x = a;
        }
        self
    }
    pub fn align_y(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_y = a;
        }
        self
    }
    pub fn align_self(mut self, a: Align) -> Self {
        if let Some(s) = self.style_mut() {
            s.align_self = Some(a);
        }
        self
    }

    // spacing
    pub fn padding(mut self, e: Edges) -> Self {
        if let Some(s) = self.style_mut() {
            s.padding = e;
        }
        self
    }
    pub fn margin(mut self, e: Edges) -> Self {
        if let Some(s) = self.style_mut() {
            s.margin = e;
        }
        self
    }
    pub fn gap(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.gap = v;
        }
        self
    }

    // position
    pub fn absolute(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.position = Position::Absolute;
        }
        self
    }
    pub fn inset(mut self, e: Edges) -> Self {
        if let Some(s) = self.style_mut() {
            s.inset = e;
        }
        self
    }

    // visuals
    pub fn background(mut self, color: Color) -> Self {
        if let Some(s) = self.style_mut() {
            s.background = Some(color);
        }
        self
    }
    pub fn border_radius(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.border_radius = v;
        }
        self
    }
    pub fn border(mut self, color: Color, thickness: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.border_color = Some(color);
            s.border_thickness = thickness;
        }
        self
    }
    pub fn opacity(mut self, v: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.opacity = v;
        }
        self
    }
    pub fn shadow(mut self, color: Color, offset_x: f32, offset_y: f32, blur: f32) -> Self {
        if let Some(s) = self.style_mut() {
            s.shadow_color = color;
            s.shadow_offset_x = offset_x;
            s.shadow_offset_y = offset_y;
            s.shadow_blur = blur;
        }
        self
    }
    pub fn overflow_hidden(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.overflow = Overflow::Hidden;
        }
        self
    }
    pub fn overflow_scroll(mut self) -> Self {
        if let Some(s) = self.style_mut() {
            s.overflow = Overflow::Scroll;
        }
        self
    }

    // font (text only)
    pub fn font(mut self, font_: Font) -> Self {
        if let Element::Text { ref mut font, .. } = self {
            *font = font_;
        }
        self
    }

    // on_click
    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        if let Element::Button {
            ref mut on_click, ..
        } = self
        {
            *on_click = Some(Box::new(f));
        }
        self
    }
}

pub fn empty() -> Element {
    Element::Empty
}

pub fn rect(color: Color) -> Element {
    Element::Rect {
        color,
        style: Style::default(),
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn text(content: &str, color: Color) -> Element {
    Element::Text {
        content: content.to_string(),
        color,
        font: Font::Default,
        style: Style::default(),
    }
}

pub fn button(label: &str) -> Element {
    Element::Button {
        label: label.to_string(),
        style: Style::default(),
        on_click: None,
        resolved_x: 0.0,
        resolved_y: 0.0,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn row(children: Vec<Element>) -> Element {
    Element::Row {
        style: Style::default(),
        children,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn column(children: Vec<Element>) -> Element {
    Element::Column {
        style: Style::default(),
        children,
        resolved_w: 0.0,
        resolved_h: 0.0,
    }
}

pub fn exit() {
    std::process::exit(0);
}

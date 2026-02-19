use crate::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum Align {
    Start,
    Center,
    End,
}

#[derive(Clone, Copy)]
pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Padding {
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

#[derive(Clone)]
pub enum Size {
    Fixed(f32),
    Fill,
    Percent(f32),
}

impl Size {
    pub fn fixed(v: f32) -> Self {
        Size::Fixed(v)
    }
    pub fn fill() -> Self {
        Size::Fill
    }
    pub fn percent(v: f32) -> Self {
        Size::Percent(v)
    }
}

#[derive(Clone, Default)]
pub struct Style {
    pub width: Option<Size>,
    pub height: Option<Size>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Padding,
    pub align_x: Align,
    pub align_y: Align,
}

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::all(0.0)
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Callbacks {
    pub on_click: Option<Box<dyn FnMut()>>,
    pub on_hover: Option<Box<dyn FnMut()>>,
    pub on_just_hovered: Option<Box<dyn FnMut()>>,
    pub on_just_unhovered: Option<Box<dyn FnMut()>>,
}

impl Callbacks {
    pub fn none() -> Self {
        Self {
            on_click: None,
            on_hover: None,
            on_just_hovered: None,
            on_just_unhovered: None,
        }
    }
}

pub enum Element {
    Rect {
        w: f32,
        h: f32,
        color: Color,
        style: Style,
        callbacks: Callbacks,
    },
    Text {
        content: String,
        color: Color,
        style: Style,
    },
    Row {
        style: Style,
        children: Vec<Element>,
    },
    Column {
        style: Style,
        children: Vec<Element>,
    },
    Empty,
}

impl Element {
    fn style_mut(&mut self) -> Option<&mut Style> {
        match self {
            Element::Rect { style, .. } => Some(style),
            Element::Text { style, .. } => Some(style),
            Element::Row { style, .. } => Some(style),
            Element::Column { style, .. } => Some(style),
            Element::Empty => None,
        }
    }

    pub fn width(mut self, s: Size) -> Self {
        if let Some(st) = self.style_mut() {
            st.width = Some(s);
        }
        self
    }

    pub fn height(mut self, s: Size) -> Self {
        if let Some(st) = self.style_mut() {
            st.height = Some(s);
        }
        self
    }

    pub fn min_width(mut self, v: f32) -> Self {
        if let Some(st) = self.style_mut() {
            st.min_width = Some(v);
        }
        self
    }

    pub fn max_width(mut self, v: f32) -> Self {
        if let Some(st) = self.style_mut() {
            st.max_width = Some(v);
        }
        self
    }

    pub fn min_height(mut self, v: f32) -> Self {
        if let Some(st) = self.style_mut() {
            st.min_height = Some(v);
        }
        self
    }

    pub fn max_height(mut self, v: f32) -> Self {
        if let Some(st) = self.style_mut() {
            st.max_height = Some(v);
        }
        self
    }

    pub fn padding(mut self, p: Padding) -> Self {
        if let Some(st) = self.style_mut() {
            st.padding = p;
        }
        self
    }

    pub fn align_x(mut self, a: Align) -> Self {
        if let Some(st) = self.style_mut() {
            st.align_x = a;
        }
        self
    }

    pub fn align_y(mut self, a: Align) -> Self {
        if let Some(st) = self.style_mut() {
            st.align_y = a;
        }
        self
    }
}

pub fn rect(w: f32, h: f32, color: Color) -> Element {
    Element::Rect {
        w,
        h,
        color,
        style: Style::new(),
        callbacks: Callbacks::none(),
    }
}

pub fn text(content: &str, color: Color) -> Element {
    Element::Text {
        content: content.to_string(),
        color,
        style: Style::new(),
    }
}

pub fn row(children: Vec<Element>) -> Element {
    Element::Row {
        style: Style::new(),
        children,
    }
}

pub fn column(children: Vec<Element>) -> Element {
    Element::Column {
        style: Style::new(),
        children,
    }
}

pub fn empty() -> Element {
    Element::Empty
}

pub fn exit() {
    std::process::exit(0);
}

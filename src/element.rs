use crate::color::Color;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ElementType {
    Row,
    Col,
    Rect,
}

#[derive(Debug)]
pub struct ElementStyle {
    // positional / layout
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_w: f32,
    pub min_h: f32,
    pub max_w: f32,
    pub max_h: f32,
    pub padding: [f32; 4],
    pub margin: [f32; 4],
    pub gap: f32,

    // visual
    pub fill: Color,
    pub border_thickness: f32,
    pub border_color: Option<Color>,
    pub border_radius: Option<f32>,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self {
            // positional
            x: 0.0,
            y: 0.0,
            w: 100.0,
            h: 100.0,
            min_w: 0.0,
            min_h: 0.0,
            max_w: 0.0,
            max_h: 0.0,
            padding: [0.0; 4],
            margin: [0.0; 4],
            gap: 0.0,

            // visual
            fill: Color::new(0.0, 0.0, 0.0, 1.0),
            border_thickness: 0.0,
            border_color: None,
            border_radius: None,
        }
    }
}

pub struct Element {
    pub id: u32,
    pub _type: ElementType,
    pub style: ElementStyle,
    pub children: Option<Vec<Element>>,
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self._type)
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            id: 0,
            _type: ElementType::Rect,
            style: ElementStyle::default(),
            children: None,
        }
    }
}

impl Element {
    // positional / layout
    pub fn w(mut self, w: f32) -> Self {
        self.style.w = w;
        self
    }
    pub fn h(mut self, h: f32) -> Self {
        self.style.h = h;
        self
    }
    pub fn x(mut self, x: f32) -> Self {
        self.style.x = x;
        self
    }
    pub fn y(mut self, y: f32) -> Self {
        self.style.y = y;
        self
    }
    pub fn max_w(mut self, max_w: f32) -> Self {
        self.style.max_w = max_w;
        self
    }
    pub fn max_h(mut self, max_h: f32) -> Self {
        self.style.max_h = max_h;
        self
    }
    pub fn min_w(mut self, min_w: f32) -> Self {
        self.style.min_w = min_w;
        self
    }
    pub fn min_h(mut self, min_h: f32) -> Self {
        self.style.min_h = min_h;
        self
    }

    // padding
    pub fn p(mut self, padding: [f32; 4]) -> Self {
        self.style.padding = padding;
        self
    }
    pub fn pt(mut self, padding_top: f32) -> Self {
        self.style.padding[0] = padding_top;
        self
    }
    pub fn pr(mut self, padding_right: f32) -> Self {
        self.style.padding[1] = padding_right;
        self
    }
    pub fn pb(mut self, padding_bottom: f32) -> Self {
        self.style.padding[2] = padding_bottom;
        self
    }
    pub fn pl(mut self, padding_left: f32) -> Self {
        self.style.padding[3] = padding_left;
        self
    }

    // margin
    pub fn m(mut self, margin: [f32; 4]) -> Self {
        self.style.margin = margin;
        self
    }
    pub fn mt(mut self, margin_top: f32) -> Self {
        self.style.margin[0] = margin_top;
        self
    }
    pub fn mr(mut self, margin_right: f32) -> Self {
        self.style.margin[1] = margin_right;
        self
    }
    pub fn mb(mut self, margin_bottom: f32) -> Self {
        self.style.margin[2] = margin_bottom;
        self
    }
    pub fn ml(mut self, margin_left: f32) -> Self {
        self.style.margin[3] = margin_left;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.style.gap = gap;
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.style.fill = color;
        self
    }

    // border
    pub fn border(mut self, thickness: f32) -> Self {
        self.style.border_thickness = thickness;
        self
    }
    pub fn border_color(mut self, color: Color) -> Self {
        self.style.border_color = Some(color);
        self
    }
    pub fn border_radius(mut self, radius: f32) -> Self {
        self.style.border_radius = Some(radius);
        self
    }

}

pub fn rect() -> Element {
    Element {
        _type: ElementType::Rect,
        ..Default::default()
    }
}

pub fn row(el: Vec<Element>) -> Element {
    Element {
        _type: ElementType::Row,
        children: Some(el),
        ..Default::default()
    }
}

pub fn col(el: Vec<Element>) -> Element {
    Element {
        _type: ElementType::Col,
        children: Some(el),
        ..Default::default()
    }
}

pub fn print_tree(el: &Element, prefix: &str, last: bool) {
    let connector = if last { "└── " } else { "├── " };
    println!("{}{}{}", prefix, connector, el);
    if let Some(children) = &el.children {
        let extension = if last { "    " } else { "│   " };
        let new_prefix = format!("{}{}", prefix, extension);
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            print_tree(child, &new_prefix, i == count - 1);
        }
    }
}

pub fn print_root(el: &Element) {
    println!("{}", el);
    if let Some(children) = &el.children {
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            print_tree(child, "", i == count - 1);
        }
    }
}

use crate::Color;

pub enum Element {
    Rect {
        w: f32,
        h: f32,
        color: Color,
    },
    Text {
        content: String,
        color: Color,
    },
    Row {
        gap: f32,
        children: Vec<Element>,
    },
    Column {
        gap: f32,
        children: Vec<Element>,
    },
    Empty,
}

pub fn empty() -> Element {
    Element::Empty
}

pub fn rect(w: f32, h: f32, color: Color) -> Element {
    Element::Rect { w, h, color }
}

pub fn text(content: &str, color: Color) -> Element {
    Element::Text { content: content.to_string(), color }
}

pub fn row(gap: f32, children: Vec<Element>) -> Element {
    Element::Row { gap, children }
}

pub fn column(gap: f32, children: Vec<Element>) -> Element {
    Element::Column { gap, children }
}

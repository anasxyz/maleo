#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod gpu;
    pub mod shape_renderer;
    pub mod text_renderer;
}

pub use render::gpu::{FrameFinisher, GpuContext, RenderFrame};
pub use render::shape_renderer::ShapeRenderer;
pub use render::text_renderer::TextRenderer;

mod app;
mod element;
mod fonts;
mod events;
mod color;

pub use app::{run, App};
pub use element::{column, rect, row, text, Element};
pub use fonts::{FontId, Fonts};
pub use events::{Events, Keyboard, Mouse};
pub use color::Color;

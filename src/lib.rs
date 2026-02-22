#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod gpu;
    pub mod shape_renderer;
    pub mod shadow_renderer;
    pub mod text_renderer;
}

pub use render::gpu::{FrameFinisher, GpuContext, RenderFrame};
pub use render::shape_renderer::ShapeRenderer;
pub use render::shadow_renderer::ShadowRenderer;
pub use render::text_renderer::TextRenderer;

mod app;
mod draw;
mod layout;
mod element;
mod fonts;
mod events;
mod color;

pub use app::{App, Settings};
pub use element::*;
pub use fonts::{FontId, FontBuilder, Fonts};
pub use events::{Events, Keyboard, Mouse, Key, key_code_to_key};
pub use color::Color;

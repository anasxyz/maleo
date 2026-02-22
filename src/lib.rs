#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod gpu;
    pub mod shadow_renderer;
    pub mod shape_renderer;
    pub mod text_renderer;
}

pub use render::gpu::{FrameFinisher, GpuContext, RenderFrame};
pub use render::shadow_renderer::ShadowRenderer;
pub use render::shape_renderer::ShapeRenderer;
pub use render::text_renderer::TextRenderer;

mod app;
mod color;
mod draw;
mod element;
mod events;
mod fonts;
mod layout;
mod state;
mod task;
pub(crate) mod widgets;

pub use app::{App, Settings};
pub use color::Color;
pub use element::*;
pub use events::{Event, Key, MouseButton, key_code_to_key};
pub use fonts::{FontBuilder, FontId, Fonts};
pub use task::Task;

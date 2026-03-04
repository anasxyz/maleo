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

mod window;
mod settings;
mod app;
mod color;

pub use settings::Settings;
pub use app::App;
pub use color::Color;

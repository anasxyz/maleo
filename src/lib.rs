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
mod mouse;

pub use app::App;
pub use mouse::MouseState;

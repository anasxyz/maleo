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
mod fonts;
mod mouse;
mod input;
mod ui;
pub mod widgets;

pub use app::App;
pub use fonts::{FontId, Fonts};
pub use mouse::MouseState;
pub use input::InputState;
pub use ui::Ui;

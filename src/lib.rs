// src/lib.rs

mod gpu;
mod shapes;
mod text;
mod app;
mod scene;
mod canvas;

pub use gpu::{GpuContext, RenderFrame, FrameFinisher};
pub use shapes::ShapeRenderer;
pub use text::TextRenderer;
pub use app::App;
pub use canvas::{Canvas, MouseState};
pub use scene::{Scene, DrawCommand};

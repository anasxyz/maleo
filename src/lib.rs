// src/lib.rs

mod gpu;
mod shapes;
mod text;
mod app;
mod scene;

pub use gpu::{GpuContext, RenderFrame};
pub use shapes::ShapeRenderer;
pub use text::TextRenderer;
pub use app::{App, Canvas};
pub use scene::{Scene, DrawCommand};

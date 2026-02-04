// src/lib.rs

mod shapes;
mod text;
mod app;

pub use shapes::ShapeRenderer;
pub use text::TextRenderer;
pub use app::{App, Canvas};

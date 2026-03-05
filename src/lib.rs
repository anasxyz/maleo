#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod gpu;
    pub mod shadow_renderer;
    pub mod shape_renderer;
    pub mod text_renderer;
    pub mod draw;
}

mod window;
mod settings;
mod app;
mod color;
mod element;
mod layout;

pub use crate::{
    app::App,
    color::Color,
    settings::WindowSettings,
    element::*,
};

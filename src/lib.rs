#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod gpu;
    pub mod shadow_renderer;
    pub mod shape_renderer;
    pub mod text_renderer;
    pub mod draw_ctx;
}

mod window;
mod settings;
mod app;
mod color;
mod element;
mod layout;
mod draw;

pub use crate::{
    app::App,
    color::{Color, rgb, rgba, hsl, hsla, hwb, hwba, hex},
    settings::WindowSettings,
    element::*,
};

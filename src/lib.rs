#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

pub mod prelude;

mod render {
    pub mod gpu;
    pub mod shadow_renderer;
    pub mod shape_renderer;
    pub mod text_renderer;
}

mod window;
mod settings;
mod app;
mod color;

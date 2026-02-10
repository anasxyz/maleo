use std::any::Any;
use std::marker::PhantomData;
use crate::{Drawer, MouseState};

mod button;
mod slider;
mod manager;

pub use button::ButtonWidget;
pub use slider::SliderWidget;
pub use manager::WidgetManager;

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

pub trait Widget: Any {
    fn id(&self) -> usize;
    fn bounds(&self) -> Rect;
    fn update(&mut self, mouse: &MouseState);
    fn render(&mut self, drawer: &mut Drawer);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug)]
pub struct WidgetHandle<T: Widget> {
    pub(crate) id: usize,
    _marker: PhantomData<T>,
}

impl<T: Widget> Copy for WidgetHandle<T> {}
impl<T: Widget> Clone for WidgetHandle<T> {
    fn clone(&self) -> Self { *self }
}

impl<T: Widget> WidgetHandle<T> {
    pub(crate) fn new(id: usize) -> Self {
        Self { id, _marker: PhantomData }
    }
}

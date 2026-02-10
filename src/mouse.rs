#[derive(Clone, Copy, Debug, Default)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,

    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,

    pub right_pressed: bool,
    pub right_just_pressed: bool,
    pub right_just_released: bool,

    pub middle_pressed: bool,
    pub middle_just_pressed: bool,
    pub middle_just_released: bool,

    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl MouseState {
    pub fn is_over(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.x >= x && self.x <= x + w && self.y >= y && self.y <= y + h
    }
}

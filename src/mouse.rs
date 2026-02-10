#[derive(Clone, Copy, Debug)]
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

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            left_pressed: false,
            left_just_pressed: false,
            left_just_released: false,
            right_pressed: false,
            right_just_pressed: false,
            right_just_released: false,
            middle_pressed: false,
            middle_just_pressed: false,
            middle_just_released: false,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }
}

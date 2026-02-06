#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left_pressed: bool,
    pub left_just_pressed: bool,
    pub left_just_released: bool,
    pub right_pressed: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            left_pressed: false,
            left_just_pressed: false,
            left_just_released: false,
            right_pressed: false,
        }
    }
}

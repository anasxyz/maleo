use std::collections::HashSet;
use winit::keyboard::KeyCode;

pub struct Input {
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,

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

    pub keys_pressed: HashSet<KeyCode>,
    pub keys_just_pressed: HashSet<KeyCode>,
    pub keys_just_released: HashSet<KeyCode>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
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
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
        }
    }
}

impl Input {
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }

    pub fn mouse_over(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.mouse_x >= x && self.mouse_x <= x + w &&
        self.mouse_y >= y && self.mouse_y <= y + h
    }

    pub(crate) fn clear_frame_state(&mut self) {
        self.mouse_dx = 0.0;
        self.mouse_dy = 0.0;
        self.left_just_pressed = false;
        self.left_just_released = false;
        self.right_just_pressed = false;
        self.right_just_released = false;
        self.middle_just_pressed = false;
        self.middle_just_released = false;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
    }
}

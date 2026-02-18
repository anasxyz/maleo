use std::collections::HashSet;
use winit::keyboard::KeyCode;

pub struct Mouse {
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

impl Mouse {
    pub fn over(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.x >= x && self.x <= x + w && self.y >= y && self.y <= y + h
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0, dx: 0.0, dy: 0.0,
            left_pressed: false, left_just_pressed: false, left_just_released: false,
            right_pressed: false, right_just_pressed: false, right_just_released: false,
            middle_pressed: false, middle_just_pressed: false, middle_just_released: false,
            scroll_x: 0.0, scroll_y: 0.0,
        }
    }
}

pub struct Keyboard {
    pub pressed: HashSet<KeyCode>,
    pub just_pressed: HashSet<KeyCode>,
    pub just_released: HashSet<KeyCode>,
}

impl Keyboard {
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
        }
    }
}

pub struct Events {
    pub mouse: Mouse,
    pub keyboard: Keyboard,
}

impl Default for Events {
    fn default() -> Self {
        Self {
            mouse: Mouse::default(),
            keyboard: Keyboard::default(),
        }
    }
}

impl Events {
    pub(crate) fn clear_frame_state(&mut self) {
        self.mouse.dx = 0.0;
        self.mouse.dy = 0.0;
        self.mouse.left_just_pressed = false;
        self.mouse.left_just_released = false;
        self.mouse.right_just_pressed = false;
        self.mouse.right_just_released = false;
        self.mouse.middle_just_pressed = false;
        self.mouse.middle_just_released = false;
        self.mouse.scroll_x = 0.0;
        self.mouse.scroll_y = 0.0;
        self.keyboard.just_pressed.clear();
        self.keyboard.just_released.clear();
    }
}

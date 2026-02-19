use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Keyboard {
    pub pressed: HashSet<Key>,
    pub just_pressed: HashSet<Key>,
    pub just_released: HashSet<Key>,
}

impl Keyboard {
    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: Key) -> bool {
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

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    LControl,
    LShift,
    LAlt,
    RControl,
    RShift,
    RAlt,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Period,
    Quote,
    Slash,
    Backslash,
    Equal,
    Minus,
    Space,
    Enter,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadDecimal,
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Enter => write!(f, "\r\n"),
            Key::Space => write!(f, " "),
            Key::Comma => write!(f, ","),
            Key::Quote => write!(f, "'"),
            Key::A => write!(f, "a"),
            Key::B => write!(f, "b"),
            Key::C => write!(f, "c"),
            Key::D => write!(f, "d"),
            Key::E => write!(f, "e"),
            Key::F => write!(f, "f"),
            Key::G => write!(f, "g"),
            Key::H => write!(f, "h"),
            Key::I => write!(f, "i"),
            Key::J => write!(f, "j"),
            Key::K => write!(f, "k"),
            Key::L => write!(f, "l"),
            Key::M => write!(f, "m"),
            Key::N => write!(f, "n"),
            Key::O => write!(f, "o"),
            Key::P => write!(f, "p"),
            Key::Q => write!(f, "q"),
            Key::R => write!(f, "r"),
            Key::S => write!(f, "s"),
            Key::T => write!(f, "t"),
            Key::U => write!(f, "u"),
            Key::V => write!(f, "v"),
            Key::W => write!(f, "w"),
            Key::X => write!(f, "x"),
            Key::Y => write!(f, "y"),
            Key::Z => write!(f, "z"),
            Key::Num0 => write!(f, "0"),
            Key::Num1 => write!(f, "1"),
            Key::Num2 => write!(f, "2"),
            Key::Num3 => write!(f, "3"),
            Key::Num4 => write!(f, "4"),
            Key::Num5 => write!(f, "5"),
            Key::Num6 => write!(f, "6"),
            Key::Num7 => write!(f, "7"),
            Key::Num8 => write!(f, "8"),
            Key::Num9 => write!(f, "9"),
            Key::Numpad0 => write!(f, "0"),
            Key::Numpad1 => write!(f, "1"),
            Key::Numpad2 => write!(f, "2"),
            Key::Numpad3 => write!(f, "3"),
            Key::Numpad4 => write!(f, "4"),
            Key::Numpad5 => write!(f, "5"),
            Key::Numpad6 => write!(f, "6"),
            Key::Numpad7 => write!(f, "7"),
            Key::Numpad8 => write!(f, "8"),
            Key::Numpad9 => write!(f, "9"),
            _ => write!(f, ""),
        }
    }
}

// winit KeyCode to my Key enum
pub fn key_code_to_key(key: KeyCode) -> Key {
    match key {
        KeyCode::KeyA => Key::A,
        KeyCode::KeyB => Key::B,
        KeyCode::KeyC => Key::C,
        KeyCode::KeyD => Key::D,
        KeyCode::KeyE => Key::E,
        KeyCode::KeyF => Key::F,
        KeyCode::KeyG => Key::G,
        KeyCode::KeyH => Key::H,
        KeyCode::KeyI => Key::I,
        KeyCode::KeyJ => Key::J,
        KeyCode::KeyK => Key::K,
        KeyCode::KeyL => Key::L,
        KeyCode::KeyM => Key::M,
        KeyCode::KeyN => Key::N,
        KeyCode::KeyO => Key::O,
        KeyCode::KeyP => Key::P,
        KeyCode::KeyQ => Key::Q,
        KeyCode::KeyR => Key::R,
        KeyCode::KeyS => Key::S,
        KeyCode::KeyT => Key::T,
        KeyCode::KeyU => Key::U,
        KeyCode::KeyV => Key::V,
        KeyCode::KeyW => Key::W,
        KeyCode::KeyX => Key::X,
        KeyCode::KeyY => Key::Y,
        KeyCode::KeyZ => Key::Z,
        KeyCode::Digit0 => Key::Num0,
        KeyCode::Digit1 => Key::Num1,
        KeyCode::Digit2 => Key::Num2,
        KeyCode::Digit3 => Key::Num3,
        KeyCode::Digit4 => Key::Num4,
        KeyCode::Digit5 => Key::Num5,
        KeyCode::Digit6 => Key::Num6,
        KeyCode::Digit7 => Key::Num7,
        KeyCode::Digit8 => Key::Num8,
        KeyCode::Digit9 => Key::Num9,
        KeyCode::Numpad0 => Key::Numpad0,
        KeyCode::Numpad1 => Key::Numpad1,
        KeyCode::Numpad2 => Key::Numpad2,
        KeyCode::Numpad3 => Key::Numpad3,
        KeyCode::Numpad4 => Key::Numpad4,
        KeyCode::Numpad5 => Key::Numpad5,
        KeyCode::Numpad6 => Key::Numpad6,
        KeyCode::Numpad7 => Key::Numpad7,
        KeyCode::Numpad8 => Key::Numpad8,
        KeyCode::Numpad9 => Key::Numpad9,
        KeyCode::NumpadDivide => Key::NumpadDivide,
        KeyCode::NumpadMultiply => Key::NumpadMultiply,
        KeyCode::NumpadSubtract => Key::NumpadSubtract,
        KeyCode::NumpadAdd => Key::NumpadAdd,
        KeyCode::NumpadDecimal => Key::NumpadDecimal,
        KeyCode::Escape => Key::Escape,
        KeyCode::F1 => Key::F1,
        KeyCode::F2 => Key::F2,
        KeyCode::F3 => Key::F3,
        KeyCode::F4 => Key::F4,
        KeyCode::F5 => Key::F5,
        KeyCode::F6 => Key::F6,
        KeyCode::F7 => Key::F7,
        KeyCode::F8 => Key::F8,
        KeyCode::F9 => Key::F9,
        KeyCode::F10 => Key::F10,
        KeyCode::F11 => Key::F11,
        KeyCode::F12 => Key::F12,
        KeyCode::F13 => Key::F13,
        KeyCode::F14 => Key::F14,
        KeyCode::F15 => Key::F15,
        KeyCode::F16 => Key::F16,
        KeyCode::F17 => Key::F17,
        KeyCode::F18 => Key::F18,
        KeyCode::F19 => Key::F19,
        KeyCode::F20 => Key::F20,
        KeyCode::F21 => Key::F21,
        KeyCode::F22 => Key::F22,
        KeyCode::F23 => Key::F23,
        KeyCode::F24 => Key::F24,
        KeyCode::ControlLeft => Key::LControl,
        KeyCode::ShiftLeft => Key::LShift,
        KeyCode::AltLeft => Key::LAlt,
        KeyCode::ControlRight => Key::RControl,
        KeyCode::ShiftRight => Key::RShift,
        KeyCode::AltRight => Key::RAlt,
        KeyCode::BracketLeft => Key::LBracket,
        KeyCode::BracketRight => Key::RBracket,
        KeyCode::Semicolon => Key::Semicolon,
        KeyCode::Comma => Key::Comma,
        KeyCode::Period => Key::Period,
        KeyCode::Quote => Key::Quote,
        KeyCode::Slash => Key::Slash,
        KeyCode::Backslash => Key::Backslash,
        KeyCode::Equal => Key::Equal,
        KeyCode::Minus => Key::Minus,
        KeyCode::Space => Key::Space,
        KeyCode::Enter => Key::Enter,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Delete => Key::Delete,
        KeyCode::Insert => Key::Insert,
        KeyCode::Home => Key::Home,
        KeyCode::End => Key::End,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::ArrowUp => Key::Up,
        KeyCode::ArrowDown => Key::Down,
        KeyCode::ArrowLeft => Key::Left,
        KeyCode::ArrowRight => Key::Right,
        _ => Key::Unknown,
    }
}

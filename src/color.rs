#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    // internal float constructor
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    // 0-255 integers
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    // hex string: "#rgb", "#rrggbb", "#rrggbbaa" 
    // # is optional and ignored
    pub fn hex(s: &str) -> Self {
        let s = s.trim_start_matches('#');
        match s.len() {
            // shorthand #rgb -> #rrggbb
            3 => {
                let r = u8::from_str_radix(&s[0..1].repeat(2), 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[1..2].repeat(2), 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[2..3].repeat(2), 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                Self::rgb(r, g, b)
            }
            8 => {
                let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
                let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
                Self::rgba(r, g, b, a)
            }
            _ => Self::BLACK,
        }
    }

    // hsl: hue 0-360, saturation 0-1, lightness 0-1
    pub fn hsl(h: f32, s: f32, l: f32) -> Self {
        Self::hsla(h, s, l, 1.0)
    }

    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        Self { r, g, b, a }
    }

    // hwb: hue 0-360, whiteness 0-1, blackness 0-1
    pub fn hwb(h: f32, w: f32, b: f32) -> Self {
        Self::hwba(h, w, b, 1.0)
    }

    pub fn hwba(h: f32, w: f32, b_val: f32, a: f32) -> Self {
        // hwb -> hsl conversion
        let w = w.min(1.0 - b_val); 
        let s = 1.0 - w / (1.0 - b_val);
        let l = (1.0 - b_val) / 2.0 + w / 2.0;
        let (r, g, b) = hsl_to_rgb(h, s, l);
        Self { r, g, b, a }
    }

    // lighten/darken helpers
    pub fn lighten(self, amount: f32) -> Self {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        Self::hsla(h, s, (l + amount).min(1.0), self.a)
    }

    pub fn darken(self, amount: f32) -> Self {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        Self::hsla(h, s, (l - amount).max(0.0), self.a)
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_array(arr: [f32; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }

    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h = h / 360.0;
    (
        hue_to_rgb(p, q, h + 1.0 / 3.0),
        hue_to_rgb(p, q, h),
        hue_to_rgb(p, q, h - 1.0 / 3.0),
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    if max == min {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };

    (h / 6.0 * 360.0, s, l)
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Self {
        Self {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

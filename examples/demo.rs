#![allow(dead_code, unused)]

use bento::{App, BentoApp, Color};

struct Demo {
}

impl BentoApp for Demo {
}

fn main() {
    App::new("Bento UI Demo", 800, 600).run(Demo {});
}

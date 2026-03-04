use bento::prelude::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}

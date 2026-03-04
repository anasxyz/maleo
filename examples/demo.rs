use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }
}

fn main() {
    MyApp::run(Settings::default());
}

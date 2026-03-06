use bento::*;

struct MyApp;

impl App for MyApp {
    fn new() -> Self {
        MyApp
    }

    fn view(&mut self) -> Element {
        col(vec![
            rect()
                .w(Size::Percent(100.0))
                .h(Size::Fixed(200.0))
                .bg(rgb(50, 200, 50)),
            rect()
                .w(Size::Fixed(100.0))
                .h(Size::Fixed(100.0))
                .bg(rgb(200, 0, 0))
                .hide(),
            rect()
                .w(Size::Fixed(150.0))
                .h(Size::Fixed(150.0))
                .bg(Color::rgba(50, 50, 200, 128))
                .z_index(10)
                .absolute()
                .top(Size::Fixed(50.0))
                .left(Size::Fixed(50.0)),
            rect()
                .w(Size::Fixed(200.0))
                .h(Size::Fixed(200.0))
                .bg(rgb(200, 200, 0))
                .z_index(-1)
                .absolute()
                .top(Size::Fixed(0.0))
                .left(Size::Fixed(0.0)),
        ])
        .w(Size::Percent(100.0))
        .h(Size::Percent(100.0))
    }
}

fn main() {
    MyApp::run(WindowSettings::default());
}

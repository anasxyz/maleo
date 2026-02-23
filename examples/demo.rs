use bento::*;

struct Counter {
    count: i32,
}

#[derive(Clone)]
enum Action {
    Increment,
    Decrement,
}

impl App for Counter {
    type Action = Action;

    fn new() -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, action: Action) -> Vec<Task<Action>> {
        match action {
            Action::Increment => self.count += 1,
            Action::Decrement => self.count -= 1,
        }
        vec![]
    }

    fn view(&self) -> Element<Action> {
        column(vec![
            text(&self.count.to_string(), Color::WHITE).font_size(48.0),
            button("−").on_click(Action::Decrement),
            button("+").on_click(Action::Increment),
        ])
        .gap(16.0)
        .background(Color::hex("#eeeeee"))
        .width(percent(100.0))
        .height(percent(100.0))
    }
}

fn main() {
    Counter::run(Settings::default().title("demo"));
}

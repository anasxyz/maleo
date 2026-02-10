use rentex::{App, Ctx, Fonts, RentexApp};
use rentex::widgets::{ButtonWidget, WidgetHandle};

struct MyApp {
    btn: Option<WidgetHandle<ButtonWidget>>,
    count: u32,
    font: Option<rentex::FontId>,
}

impl RentexApp for MyApp {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("Arial", 18.0);
        self.font = Some(font);

        let btn = ctx.widgets.button("Click me!");
        ctx.widgets.get_mut(btn)
            .position(300.0, 280.0)
            .font(font)
            .auto_size()
            .color([0.2, 0.4, 0.9, 1.0]);
        self.btn = Some(btn);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.widgets.get(self.btn.unwrap()).just_clicked {
            self.count += 1;
            ctx.widgets.get_mut(self.btn.unwrap()).text(format!("Clicked: {} times", self.count));
        }
    }
}

fn main() {
    App::new("Counter", 800, 600).run(Fonts::new(), MyApp {
        btn: None,
        count: 0,
        font: None,
    });
}

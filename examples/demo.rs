use rentex::widgets::{ButtonWidget, SliderWidget, WidgetHandle};
use rentex::{App, Ctx, Fonts, RentexApp};

struct MyApp {
    btn: Option<WidgetHandle<ButtonWidget>>,
    btn2: Option<WidgetHandle<ButtonWidget>>,
    slider: Option<WidgetHandle<SliderWidget>>,
    count: u32,
}

impl RentexApp for MyApp {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("JetBrainsMono Nerd Font", 16.0);

        let btn = ctx.widgets.button("Click me");
        ctx.widgets
            .get_mut(btn)
            .font(font)
            .auto_size()
            .color([0.2, 0.2, 0.2, 1.0]);
        self.btn = Some(btn);

        let btn2 = ctx.widgets.button("Click me too");
        ctx.widgets
            .get_mut(btn2)
            .font(font)
            .auto_size()
            .color([0.2, 0.4, 0.9, 1.0]);
        self.btn2 = Some(btn2);

        let slider = ctx.widgets.slider();
        ctx.widgets
            .get_mut(slider)
            .size(200.0, 20.0)
            .range(0.0, 100.0)
            .value(50.0)
            .show_label(font);
        self.slider = Some(slider);

        ctx.layout
            .hstack()
            .position(0.0, 0.0)
            .padding(4.0)
            .gap(2.0)
            .add(btn)
            .add(btn2)
            .add(slider);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.widgets.get(self.slider.unwrap()).just_changed {
            self.count = ctx.widgets.get(self.slider.unwrap()).value as u32;
            ctx.widgets
                .get_mut(self.btn.unwrap())
                .text(format!("Value: {}", self.count));
        }

        if ctx.widgets.get(self.btn2.unwrap()).just_clicked {
            ctx.exit();
        }
    }
}

fn main() {
    App::new("Counter", 800, 600).run(
        Fonts::new(),
        MyApp {
            btn: None,
            btn2: None,
            slider: None,
            count: 0,
        },
    );
}

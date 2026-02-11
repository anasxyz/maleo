#![allow(dead_code, unused)]

use bento::widgets::{ButtonWidget, SliderWidget, TextInputWidget, WidgetHandle};
use bento::{App, Ctx, FontId, Fonts, BentoApp};

struct Demo {
    font: Option<FontId>,

    // top row
    search_input: Option<WidgetHandle<TextInputWidget>>,
    search_btn:   Option<WidgetHandle<ButtonWidget>>,

    // middle: two columns side by side
    name_input:   Option<WidgetHandle<TextInputWidget>>,
    email_input:  Option<WidgetHandle<TextInputWidget>>,
    vol_slider:   Option<WidgetHandle<SliderWidget>>,
    bright_slider: Option<WidgetHandle<SliderWidget>>,

    // bottom action row
    ok_btn:     Option<WidgetHandle<ButtonWidget>>,
    cancel_btn: Option<WidgetHandle<ButtonWidget>>,
    reset_btn:  Option<WidgetHandle<ButtonWidget>>,
}

impl BentoApp for Demo {
    fn setup(&mut self, ctx: &mut Ctx) {
        let font = ctx.fonts.add("JetBrainsMono Nerd Font", 14.0);
        self.font = Some(font);

        let search_input = ctx.widgets.text_input();
        ctx.widgets.get_mut(search_input)
            .font(font).size(260.0, 32.0).placeholder("search...");
        self.search_input = Some(search_input);

        let search_btn = ctx.widgets.button("go");
        ctx.widgets.get_mut(search_btn)
            .font(font).size(52.0, 32.0)
            .color([0.18, 0.48, 0.92, 1.0])
            .hover_color([0.25, 0.58, 1.0, 1.0])
            .press_color([0.12, 0.35, 0.75, 1.0]);
        self.search_btn = Some(search_btn);

        let name_input = ctx.widgets.text_input();
        ctx.widgets.get_mut(name_input)
            .font(font).size(180.0, 32.0).placeholder("name...");
        self.name_input = Some(name_input);

        let email_input = ctx.widgets.text_input();
        ctx.widgets.get_mut(email_input)
            .font(font).size(180.0, 32.0).placeholder("email...");
        self.email_input = Some(email_input);

        let vol_slider = ctx.widgets.slider();
        ctx.widgets.get_mut(vol_slider)
            .size(180.0, 18.0).range(0.0, 100.0).value(70.0)
            .fill_color([0.18, 0.48, 0.92, 1.0]);
        self.vol_slider = Some(vol_slider);

        let bright_slider = ctx.widgets.slider();
        ctx.widgets.get_mut(bright_slider)
            .size(180.0, 18.0).range(0.0, 100.0).value(40.0)
            .fill_color([0.85, 0.55, 0.1, 1.0]);
        self.bright_slider = Some(bright_slider);

        let ok_btn = ctx.widgets.button("ok");
        ctx.widgets.get_mut(ok_btn)
            .font(font).size(88.0, 32.0)
            .color([0.12, 0.42, 0.22, 1.0])
            .hover_color([0.18, 0.58, 0.32, 1.0])
            .press_color([0.08, 0.28, 0.15, 1.0]);
        self.ok_btn = Some(ok_btn);

        let cancel_btn = ctx.widgets.button("cancel");
        ctx.widgets.get_mut(cancel_btn)
            .font(font).size(88.0, 32.0)
            .color([0.38, 0.12, 0.12, 1.0])
            .hover_color([0.52, 0.18, 0.18, 1.0])
            .press_color([0.25, 0.08, 0.08, 1.0]);
        self.cancel_btn = Some(cancel_btn);

        let reset_btn = ctx.widgets.button("reset");
        ctx.widgets.get_mut(reset_btn)
            .font(font).size(88.0, 32.0)
            .color([0.22, 0.22, 0.22, 1.0])
            .hover_color([0.32, 0.32, 0.32, 1.0])
            .press_color([0.14, 0.14, 0.14, 1.0]);
        self.reset_btn = Some(reset_btn);

        let search_row = ctx.layout
            .hstack()
            .gap(8.0)
            .add(search_input)
            .add(search_btn)
            .as_ref();

        let col_a = ctx.layout
            .vstack()
            .gap(10.0)
            .add(name_input)
            .add(email_input)
            .as_ref();

        let col_b = ctx.layout
            .vstack()
            .gap(18.0)
            .add(vol_slider)
            .add(bright_slider)
            .as_ref();

        let columns = ctx.layout
            .hstack()
            .gap(16.0)
            .add_container(col_a)
            .add_container(col_b)
            .as_ref();

        let action_row = ctx.layout
            .hstack()
            .gap(8.0)
            .add(ok_btn)
            .add(cancel_btn)
            .add(reset_btn)
            .as_ref();

        ctx.layout
            .vstack()
            .position(40.0, 40.0)
            .gap(20.0)
            .add_container(search_row)
            .add_container(columns)
            .add_container(action_row);
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if ctx.widgets.get(self.ok_btn.unwrap()).just_clicked {
            ctx.widgets.get_mut(self.name_input.unwrap()).value("");
            ctx.widgets.get_mut(self.email_input.unwrap()).value("");
        }

        if ctx.widgets.get(self.reset_btn.unwrap()).just_clicked {
            ctx.widgets.get_mut(self.vol_slider.unwrap()).value(70.0);
            ctx.widgets.get_mut(self.bright_slider.unwrap()).value(40.0);
        }
    }
}

fn main() {
    App::new("bento", 440, 260).run(
        Fonts::new(),
        Demo {
            font: None,
            search_input: None,
            search_btn: None,
            name_input: None,
            email_input: None,
            vol_slider: None,
            bright_slider: None,
            ok_btn: None,
            cancel_btn: None,
            reset_btn: None,
        },
    );
}

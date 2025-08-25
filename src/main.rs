use crate::app::App;
use sdl2::pixels::Color;
use sdl2::{event::EventPollIterator};
use std::time::Duration;

mod app;

struct Entity {}

impl Entity {
    fn render(&self) {}
    fn input(&self, _event_pump: &mut EventPollIterator) {}
    fn update(&self) {}
}

fn main() {
    let (mut app, mut event_pump) = App::new("Meu ovo", 450, 450);
    let fps = 60;

    loop {
        app.get_window().set_draw_color(Color::RGB(0, 0, 0));
        app.get_window().clear();

        if app.running {
            break;
        };

        if let Some(entity) = app.entitys.first_mut() {
            let mut event = event_pump.poll_iter();
            entity.input(&mut event);
        }

        for entity in &app.entitys {
            entity.render();
        }

        for entity in &app.entitys {
            entity.update();
        }

        app.get_window().present();

        frame_hate!(fps);
    }

    println!("Hello, world!");
}

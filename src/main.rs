use sdl2::event::Event;
use crate::app::app::App;
use app::game_object::GameObject;
use sdl2::pixels::Color;
use std::time::Duration;

mod app;

struct Entity {}

impl GameObject for Entity {
    fn render(&self) {}
    fn input(&self, _event_pump: &mut Event) {}
    fn update(&self) {}
}

fn main() {
    let mut app = App::new("Meu ovo", 450, 450);
    let fps = 60;

    'running: loop {
        app.get_window().set_draw_color(Color::RGB(0, 0, 0));
        app.get_window().clear();

        if !app.running {
            break 'running;
        };

        app.event_player();

        for entity in &app.entitys {
            entity.render();
        }

        for entity in &app.entitys {
            entity.update();
        }

        app.get_window().present();

        frame_hate!(fps);
    }
}

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[allow(dead_code)]
pub trait GameObject<T>: Update + Draw + Control + CollisionArea<T> {}

pub trait CollisionArea<T> {
        #[allow(dead_code)]
        fn get_collision_body(&mut self) -> Option<&mut T> {
                None
        }

        #[allow(dead_code)]
        fn check_collision(&mut self, _entity: &mut T) {}

        #[allow(dead_code)]
        fn set_collision_status(&mut self, _state: bool) {}
}

pub trait Control {
        #[allow(dead_code)]
        fn input(&mut self, _event_pump: &mut Event) {}

        #[allow(dead_code)]
        fn set_position(&mut self, _x: i32, _y: i32) {}
}

pub trait Update {
        #[allow(dead_code)]
        fn update(&mut self) {}
}

pub trait Draw {
        #[allow(dead_code)]
        fn set_color(&mut self, _r: u8, _g: u8, _b: u8) {}

        #[allow(dead_code)]
        fn get_color(&self) -> Color {
                Color {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 125,
                }
        }

        #[allow(dead_code)]
        fn get_draw_rect(&self) -> Option<&Rect> {
                None
        }
}

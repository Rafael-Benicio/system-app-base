use sdl2::event::Event;

pub trait GameObject {
    fn render(&self) {}
    fn input(&self, _event_pump: &mut Event) {}
    fn update(&self) {}
}

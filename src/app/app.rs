use sdl2::keyboard::Keycode;
use crate::Event::KeyDown;
use crate::app::game_object::GameObject;
use crate::{Color, draw_simple_rect, keydown};

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, Sdl, VideoSubsystem};

use std::collections::HashMap;
use std::path::Path;

pub struct App<'a, T> {
    window: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    texture: HashMap<String, Texture<'a>>,
    event_pump: EventPump,
    pub entitys: Vec<Box<dyn GameObject<T>>>,
    pub running: bool,
}

impl<'a, T> App<'a, T> {
    pub fn new(title: &str, width: u32, height: u32) -> App<'a, T> {
        let sdl_context: Sdl = sdl2::init().expect("Erro in sdl2 init");
        let video_subsystem: VideoSubsystem = sdl_context
            .video()
            .expect("Erro in VideoSubsystem creation");

        let window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .expect("Erro in window creation")
            .into_canvas()
            .build()
            .expect("Erro in GameState creation");

        let event_pump = sdl_context.event_pump().unwrap();

        App {
            texture: Default::default(),
            texture_creator: window.texture_creator(),
            window,
            event_pump,
            entitys: vec![],
            running: true,
        }
    }

    #[allow(dead_code)]
    pub fn load_image(&'a mut self, path: &str) {
        match self.texture_creator.load_texture(Path::new(&path)) {
            Ok(txr) => self.texture.insert(path.to_string(), txr),
            Err(_) => {
                panic!("Não conseguiu carregar")
            }
        };
    }

    #[allow(dead_code)]
    pub fn get_textures(&mut self) -> &mut HashMap<String, Texture<'a>> {
        &mut self.texture
    }

    #[allow(dead_code)]
    pub fn input_itens(&mut self) {
        for mut event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | keydown!(Keycode::Escape) => self.running = false,
                _ => {
                    if let Some(entity) = self.entitys.first_mut() {
                        entity.input(&mut event)
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn render_itens(&mut self) {
        self.window.set_draw_color(Color::RGB(0, 0, 0));
        self.window.clear();

        for entity in self.entitys.iter_mut() {
            draw_simple_rect!(self.window, *entity.get_draw_rect(), entity.get_color());
        }

        self.window.present();
    }

    #[allow(dead_code)]
    pub fn collision_itens(&mut self) {
        for entity in self.entitys.iter_mut() {
            entity.set_collision_status(false);
        }

        if self.entitys.len() >= 2 {
            let e = self.entitys.len();

            for head in 1..e {
                let (left, right) = self.entitys.split_at_mut(head);
                let last = left.len() - 1;

                let entity_1 = &mut left[last];

                for i in 0..right.len() {
                    let entity_2 = &mut right[i];

                    let rect = entity_2.get_collision_body();

                    if rect.is_none() {
                        continue;
                    }

                    entity_1.check_collision(rect.unwrap());
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_itens(&mut self) {
        for entity in self.entitys.iter_mut() {
            entity.update();
        }
    }
}

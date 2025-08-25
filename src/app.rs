use crate::Entity;
use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::image::LoadTexture;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::Path;

pub struct App<'a> {
    window: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    texture: HashMap<String, Texture<'a>>,
    pub entitys: Vec<Entity>,
    pub running: bool,
}
impl<'a> App<'a> {
    pub fn new(title: &str, width: u32, height: u32) -> (App<'a>, EventPump) {
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

        (
            App {
                texture: Default::default(),
                texture_creator: window.texture_creator(),
                window,
                entitys: vec![],
                running: true,
            },
            event_pump,
        )
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

    pub fn get_window(&mut self) -> &mut Canvas<sdl2::video::Window> {
        &mut self.window
    }

    #[allow(dead_code)]
    pub fn get_textures(&mut self) -> &mut HashMap<String, Texture<'a>> {
        &mut self.texture
    }
}

#[macro_export]
macro_rules! frame_hate {
    ($a:expr) => {
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / $a));
    };
}

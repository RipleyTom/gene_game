
use crate::creature::CreatureType;
use crate::creaturemap::CreatureMap;
use crate::world::World;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator};
use sdl2::Sdl;

pub struct Renderer<'ctx, 'texture> {
    size_x: u32,
    size_y: u32,

    sdl_ctx: &'ctx Sdl,
    vid: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture: Option<sdl2::render::Texture<'texture>>,
}
impl<'ctx, 'texture> Renderer<'ctx, 'texture> {
    pub fn new(sdl_ctx: &'ctx Sdl, world: &World) -> Renderer<'ctx, 'texture> {

        let video_subsystem = sdl_ctx.video().unwrap();
        let (size_x, size_y) = world.get_size();

        let window = video_subsystem
            .window("Gene Game", size_x, size_y)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .target_texture()
//            .present_vsync()
            .build()
            .unwrap();

        canvas.clear();
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.present();

        Renderer {
            size_x: size_x,
            size_y: size_y,
            sdl_ctx: sdl_ctx,
            vid: video_subsystem,
            canvas: canvas,
            texture: None,
        }
    }

    pub fn get_canvas(&self) -> &sdl2::render::Canvas<sdl2::video::Window> {
        &self.canvas
    }

    pub fn init(&mut self, texture_creator: &'texture TextureCreator<sdl2::video::WindowContext>) {
        self.texture = Some(
            texture_creator
                .create_texture_streaming(PixelFormatEnum::RGBA8888, self.size_x, self.size_y)
                .map_err(|e| e.to_string())
                .unwrap(),
        );
    }

    pub fn update(&mut self, world: &World, creatures: &CreatureMap) {
        let tex = &self.texture;

        let (s_x, s_y) = (self.size_x, self.size_y);

        self.texture
            .as_mut()
            .unwrap()
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                for y in 0..s_x {
                    for x in 0..s_y {
                        let tile = world.get_tile(x, y);
                        if let None = tile.creature {
                            buffer[((y * s_y) + x) as usize * 4] = 255;
                            buffer[(((y * s_y) + x) as usize * 4) + 1] = {
                                if tile.food >= 255 {
                                    255
                                } else {
                                    tile.food as u8
                                }
                            };
                            buffer[(((y * s_y) + x) as usize * 4) + 2] = 0;
                            buffer[(((y * s_y) + x) as usize * 4) + 3] = 0;
                        } else {
                            let c = creatures
                                .get_creature(tile.creature.clone().unwrap())
                                .unwrap();
                            let (r, g, b): (u8, u8, u8);
                            match c.get_type() {
                                CreatureType::Carnivore => {
                                r = 255;
                                g = 0;
                                b = 0;
                                },
                                CreatureType::Herbivore => {
                                r = 0;
                                g = 255;
                                b = 0;
                                },
                                CreatureType::Omnivore => {
                                    r = 255;
                                    g = 255;
                                    b = 0;
                                },
                            }

                            buffer[((y * s_y) + x) as usize * 4] = 255;
                            buffer[(((y * s_y) + x) as usize * 4) + 1] = b;
                            buffer[(((y * s_y) + x) as usize * 4) + 2] = g;
                            buffer[(((y * s_y) + x) as usize * 4) + 3] = r;

                        }
                    }
                }
            })
            .unwrap();

        let rect = Rect::new(0, 0, s_x, s_y);
        self.canvas
            .copy(self.texture.as_ref().unwrap(), None, Some(rect))
            .unwrap();
        self.canvas.present();
    }
}
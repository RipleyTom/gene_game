use crate::creature::command::NUM_MAX_GENES;
use crate::creature::CreatureType;
use crate::creaturemap::CreatureMap;
use crate::world::World;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::Sdl;

pub enum DisplayTypes {
    FoodType,
    GeneComplexity,
}

pub struct Renderer<'texture> {
    width: u32,
    height: u32,

    // sdl_ctx: &'ctx Sdl,
    // vid: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture: Option<sdl2::render::Texture<'texture>>,
    gene_colors: [u8; NUM_MAX_GENES],
}
impl<'texture> Renderer<'texture> {
    pub fn new(sdl_ctx: &Sdl, world: &World) -> Renderer<'texture> {

        let video_subsystem = sdl_ctx.video().unwrap();
        let (width, height) = world.get_size();

        let window = video_subsystem
            .window("Gene Game", width, height)
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

        let mut disp_genes: [u8; NUM_MAX_GENES] = [0; NUM_MAX_GENES];

        for x in 0..NUM_MAX_GENES {
            disp_genes[x] = ((255 / (NUM_MAX_GENES + 1)) * (x + 1)) as u8;
        }

        Renderer {
            width: width,
            height: height,
            // sdl_ctx: sdl_ctx,
            // vid: video_subsystem,
            canvas: canvas,
            texture: None,
            gene_colors: disp_genes,
        }
    }

    pub fn get_canvas(&self) -> &sdl2::render::Canvas<sdl2::video::Window> {
        &self.canvas
    }

    pub fn init(&mut self, texture_creator: &'texture TextureCreator<sdl2::video::WindowContext>) {
        self.texture = Some(
            texture_creator
                .create_texture_streaming(PixelFormatEnum::RGBA8888, self.width, self.height)
                .map_err(|e| e.to_string())
                .unwrap(),
        );
    }

    pub fn update(&mut self, world: &World, creatures: &CreatureMap, disp: &DisplayTypes) {
        let (width, height) = (self.width, self.height);

        match disp {
            DisplayTypes::FoodType => {
                self.texture
                    .as_mut()
                    .unwrap()
                    .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                        for y in 0..height {
                            for x in 0..width {
                                let tile = world.get_tile(x, y);
                                if let None = tile.creature {
                                    buffer[((y * width) + x) as usize * 4] = 255;
                                    buffer[(((y * width) + x) as usize * 4) + 1] = {
                                        if tile.food >= 255 {
                                            255
                                        } else {
                                            tile.food as u8
                                        }
                                    };
                                    buffer[(((y * width) + x) as usize * 4) + 2] = 0;
                                    buffer[(((y * width) + x) as usize * 4) + 3] = 0;
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
                                        }
                                        CreatureType::Herbivore => {
                                            r = 0;
                                            g = 255;
                                            b = 0;
                                        }
                                        CreatureType::Omnivore => {
                                            r = 255;
                                            g = 255;
                                            b = 0;
                                        }
                                    }

                                    buffer[((y * width) + x) as usize * 4] = 255;
                                    buffer[(((y * width) + x) as usize * 4) + 1] = b;
                                    buffer[(((y * width) + x) as usize * 4) + 2] = g;
                                    buffer[(((y * width) + x) as usize * 4) + 3] = r;
                                }
                            }

                        }
                    })
                    .unwrap();
            }
            DisplayTypes::GeneComplexity => {
                let colors = self.gene_colors;
                self.texture
                    .as_mut()
                    .unwrap()
                    .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                        for y in 0..height {
                            for x in 0..width {
                                let tile = world.get_tile(x, y);
                                if let None = tile.creature {
                                    buffer[((y * width) + x) as usize * 4] = 255;
                                    buffer[(((y * width) + x) as usize * 4) + 1] = {
                                        if tile.food >= 255 {
                                            255
                                        } else {
                                            tile.food as u8
                                        }
                                    };
                                    buffer[(((y * width) + x) as usize * 4) + 2] = 0;
                                    buffer[(((y * width) + x) as usize * 4) + 3] = 0;
                                } else {
                                    let c = creatures
                                        .get_creature(tile.creature.clone().unwrap())
                                        .unwrap();
                                    let (r, g, b): (u8, u8, u8);
                                    r = 0;
                                    g = colors[c.get_num_genes() - 1];
                                    b = 0;

                                    buffer[((y * width) + x) as usize * 4] = 255;
                                    buffer[(((y * width) + x) as usize * 4) + 1] = b;
                                    buffer[(((y * width) + x) as usize * 4) + 2] = g;
                                    buffer[(((y * width) + x) as usize * 4) + 3] = r;
                                }

                            }

                        }
                    })
                    .unwrap();
            }
        }


        let rect = Rect::new(0, 0, width, height);
        self.canvas
            .copy(self.texture.as_ref().unwrap(), None, Some(rect))
            .unwrap();
        self.canvas.present();
    }
}
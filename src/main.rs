mod creature;
mod creaturemap;

mod renderer;
mod world;
use creature::command::Command;
use creaturemap::{CreatureId, CreatureMap};
use renderer::Renderer;
use world::World;

use rand::Rng;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::TextureCreator;
use sdl2::Sdl;

struct Game<'texture> {
    world: World,
    creatures: CreatureMap,
    pub gfx: Renderer<'texture>,
    round: u32,
}
impl<'texture> Game<'texture> {
    pub fn new(width: u32, height: u32, num_creatures: u32, sdl_ctx: &Sdl) -> Game {

        let mut world = World::new(width, height);
        let mut creatures = CreatureMap::new();
        let gfx = Renderer::new(&sdl_ctx, &world);

        for _ in 0..num_creatures {
            loop {
                let x = rand::thread_rng().gen_range(0, width);
                let y = rand::thread_rng().gen_range(0, height);

                let tile = world.get_tile_mut(x, y);

                if let None = tile.creature {
                    let id = creatures.add_creature(x, y, vec![Command::Eat, Command::Reproduce]);
                    tile.creature = Some(id);
                    break;
                }
            }
        }

        Game {
            world: world,
            creatures: creatures,
            gfx: gfx,
            round: 0,
        }
    }

    pub fn init(&mut self, tex_creat: &'texture TextureCreator<sdl2::video::WindowContext>) {
        self.gfx.init(tex_creat);
    }

    pub fn update_gfx(&mut self) {
        self.gfx.update(&self.world, &self.creatures);
    }

    pub fn display_tile_info(&self, x: i32, y: i32) {
        let (world_w, world_h) = self.world.get_size();

        if x < 0 || x >= world_w as i32 || y < 0 || y >= world_h as i32 {
            return;
        }

        let x = x as u32;
        let y = y as u32;

        let tile = self.world.get_tile(x, y);

        println!("X: {} Y: {}", x, y);
        println!("{}", tile);
        if let Some(c) = &tile.creature {
            let creat = self.creatures.get_creature(c.clone()).unwrap();
            println!("{}", creat);
        }
    }

    pub fn simulate(&mut self) -> bool {
        let mut active_creatures: Vec<CreatureId> = Vec::new();

        for i in 0..self.creatures.get_num() {
            if let Some(c) = self.creatures.get_creatureid_by_index(i) {
                active_creatures.push(c);
            }
        }
        println!(
            "Round {}, number of active creatures: {}",
            self.round,
            active_creatures.len()
        );

        debug_assert_eq!(
            active_creatures.len() as u32,
            self.world.get_num_creatures() as u32
        );

        if active_creatures.len() == 0 {
            println!("Every creature died at round {}", self.round);
            return false;
        }

        for id in &active_creatures {
            if let Some(c) = self.creatures.move_creature(id.clone()) {
                let mut creat = c.clone();
                if creat.simulate(&mut self.world, &mut self.creatures) {
                    self.creatures.set_creature(creat.get_id(), creat);
                }
            }
        }
        self.round += 1;

        active_creatures.clear();
        true
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let tex_creat;

    let mut event_pump = sdl_context.event_pump().unwrap();

    {
        let mut g = Game::new(800, 600, 500, &sdl_context);
        let main_canvas = g.gfx.get_canvas();

        // TextureCreator had to be made outside of the scope of Game as it's borrowed for the lifetime of Texture
        // Could use rental crate?
        tex_creat = main_canvas.texture_creator();

        g.init(&tex_creat);
        g.update_gfx();

        let mut paused = false;

        'running: loop {
            if !paused {
                if !g.simulate() {
                    break 'running;
                }
                g.update_gfx();
            }

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        paused = !paused;
                    }
                    Event::MouseButtonDown {
                        mouse_btn: MouseButton::Left,
                        x,
                        y,
                        ..
                    } => {
                        g.display_tile_info(x, y);
                    }
                    _ => {}
                }
            }
        }
    }
}

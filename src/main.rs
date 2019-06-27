mod creature;
mod creaturemap;
mod world;


use creature::command::Command;
use creaturemap::{CreatureId, CreatureMap};

use rand::Rng;
use world::World;
struct Game {
    world: World,
    creatures: CreatureMap,
}

impl Game {
    pub fn new(width: u32, height: u32, num_creatures: u32) -> Game {
        let mut g = Game {
            world: World::new(width, height),
            creatures: CreatureMap::new(),
        };

        for _ in 0..num_creatures {
            loop {
                let x = rand::thread_rng().gen_range(0, width);
                let y = rand::thread_rng().gen_range(0, height);

                let tile = g.world.get_tile_mut(x, y);

                if let None = tile.creature {
                    let id = g
                        .creatures
                        .add_creature(x, y, vec![Command::Eat, Command::Reproduce]);
                    tile.creature = Some(id);
                    break;
                }
            }
        }
        g
    }

    pub fn simulate(&mut self) {
        let mut round = 0;
        let mut active_creatures: Vec<CreatureId> = Vec::new();

        loop {
            for i in 0..self.creatures.get_num() {
                match self.creatures.get_creatureid_by_index(i) {
                    Some(c) => {
                        active_creatures.push(c);
                    }
                    None => {}
                }
            }

            println!(
                "Round {}, number of active creatures: {}",
                round,
                active_creatures.len()
            );
            if active_creatures.len() == 0 {
                println!("Every creature died at round {}", round);
                break;
            }

            for id in &active_creatures {
                if let Some(c) = self.creatures.get_creature_mut(id.clone()) {
                    let mut creat = c.clone();
                    if creat.simulate(&mut self.world, &mut self.creatures) {
                        self.creatures.set_creature(creat.get_id(), creat);
                    }
                }
            }

            round += 1;

            active_creatures.clear();
        }
    }
}

fn main() {
    let mut g = Game::new(1000, 1000, 500);
    g.simulate();
}

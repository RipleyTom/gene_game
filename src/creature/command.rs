
use crate::creature::{Creature, CreatureStats};
use crate::creaturemap::CreatureMap;
use crate::world::World;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::Rng;
#[derive(Clone, FromPrimitive)]
pub enum Command {
    Nop = 0,
    LookForFood,
    LookForCreature,
    Move,
    Eat,
    Attack,
    Invert,
    Reproduce,
}

impl Command {
    pub fn execute(
        &self,
        world: &mut World,
        stats: &mut CreatureStats,
        gene: &Vec<Command>,
        creatures: &mut CreatureMap,
    ) {
        match self {
            Command::Nop => {}
            Command::LookForFood => Command::c_lookfood(world, stats),
            Command::LookForCreature => Command::c_lookcreatures(world, stats),
            Command::Move => Command::c_move(world, stats),
            Command::Eat => Command::c_eat(world, stats),
            Command::Attack => Command::c_attack(world, stats, creatures),
            Command::Reproduce => Command::c_reproduce(world, stats, gene, creatures),
            Command::Invert => Command::c_invert(stats),
        }
    }

    fn c_lookfood(world: &World, stats: &mut CreatureStats) {
        let update_for_dir = |pos_x: u32, pos_y: u32, dir_x: i8, dir_y: i8| -> u8 {
            let (want_x, want_y) = Creature::adjust_pos(world, (pos_x, pos_y), (dir_x, dir_y));
            let tile = world.get_tile(want_x, want_y);
            if tile.food >= 255 {
                255
            } else {
                tile.food as u8
            }
        };

        let (x, y) = (stats.pos_x, stats.pos_y);

        stats.e = update_for_dir(x, y, 1, 0);
        stats.w = update_for_dir(x, y, -1, 0);
        stats.n = update_for_dir(x, y, 0, -1);
        stats.s = update_for_dir(x, y, 0, 1);
    }

    fn c_lookcreatures(world: &World, stats: &mut CreatureStats) {
        let update_for_dir = |pos_x: u32, pos_y: u32, dir_x: i8, dir_y: i8| -> u8 {
            let (want_x, want_y) = Creature::adjust_pos(world, (pos_x, pos_y), (dir_x, dir_y));
            let tile = world.get_tile(want_x, want_y);
            if tile.creature == None {
                0
            } else {
                255
            }
        };

        let (x, y) = (stats.pos_x, stats.pos_y);

        stats.e = update_for_dir(x, y, 1, 0);
        stats.w = update_for_dir(x, y, -1, 0);
        stats.n = update_for_dir(x, y, 0, -1);
        stats.s = update_for_dir(x, y, 0, 1);
    }

    fn c_move(world: &mut World, stats: &mut CreatureStats) {
        let dir = stats.get_proba_dir();

        let (want_x, want_y) = Creature::adjust_pos(world, (stats.pos_x, stats.pos_y), dir);

        if let None = world.get_tile(want_x, want_y).creature {
            world.get_tile_mut(stats.pos_x, stats.pos_y).creature = None;
            world.get_tile_mut(want_x, want_y).creature = Some(stats.id.clone());
            stats.pos_x = want_x;
            stats.pos_y = want_y;
        }
    }

    fn c_eat(world: &mut World, stats: &mut CreatureStats) {
        let dir = stats.get_proba_dir();

        let (want_x, want_y) = Creature::adjust_pos(world, (stats.pos_x, stats.pos_y), dir);
        let mut tile = world.get_tile_mut(want_x, want_y);

        if let Some(_) = tile.creature {
            return;
        }

        if tile.food >= 10 {
            stats.energy += 10;
            tile.food -= 10;
        } else {
            stats.energy += tile.food;
            tile.food = 0;
        }
    }

    fn c_attack(world: &mut World, stats: &mut CreatureStats, creatures: &mut CreatureMap) {
        let dir = stats.get_proba_dir();

        let (want_x, want_y) = Creature::adjust_pos(world, (stats.pos_x, stats.pos_y), dir);
        let mut tile = world.get_tile_mut(want_x, want_y);

        if let None = tile.creature {
            return;
        }

        let id_victim = tile.creature.clone().unwrap();
        let victim = creatures.get_creature_mut(id_victim.clone()).unwrap();
        if victim.stats.energy < 50 {
            // Kills it
            stats.energy += victim.stats.energy;
            tile.creature = None;
            creatures.deallocate(id_victim);
        } else {
            victim.stats.energy -= 50;
            stats.energy += 50;
        }
    }

    fn c_reproduce(
        world: &mut World,
        stats: &mut CreatureStats,
        gene: &Vec<Command>,
        creatures: &mut CreatureMap,
    ) {
        if stats.energy < 200 {
            return;
        }

        let dir = stats.get_proba_dir();

        let (want_x, want_y) = Creature::adjust_pos(world, (stats.pos_x, stats.pos_y), dir);
        let mut tile = world.get_tile_mut(want_x, want_y);

        if let Some(_) = tile.creature {
            return;
        }

        let mut new_genes = gene.clone();

        const MUTATIONCHANCE: u32 = 10;
        const NEWGENECHANCE: u32 = 1;

        if rand::thread_rng().gen_range(0, 100) < MUTATIONCHANCE {
            let new_command = rand::thread_rng().gen_range(0, 8);

            let newdiceroll = rand::thread_rng().gen_range(0, 100);
            if new_genes.len() < 16 && newdiceroll < NEWGENECHANCE {
                new_genes.push(FromPrimitive::from_u32(new_command).unwrap());
            } else {
                let selectdiceroll = rand::thread_rng().gen_range(0, new_genes.len());
                new_genes[selectdiceroll] = FromPrimitive::from_u32(new_command).unwrap();
            }
        }

        let id = creatures.add_creature(want_x, want_y, new_genes);
        tile.creature = Some(id);

        stats.energy -= 100;
    }

    fn c_invert(stats: &mut CreatureStats) {
        fn invert_stat(v: &mut u8) {
            let ret = (*v as i16) - 255;
            *v = ret.abs() as u8;
        }

        invert_stat(&mut stats.e);
        invert_stat(&mut stats.w);
        invert_stat(&mut stats.n);
        invert_stat(&mut stats.s);
    }
}
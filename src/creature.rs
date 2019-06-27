pub mod command;

use crate::creaturemap::{CreatureId, CreatureMap};
use crate::world::World;
use command::Command;
use rand::Rng;

#[derive(Clone)]
pub struct CreatureStats {
    id: CreatureId,
    pos_x: u32,
    pos_y: u32,
    energy: u32,
    e: u8,
    w: u8,
    n: u8,
    s: u8,
}
impl CreatureStats {
    pub fn get_proba_dir(&self) -> (i8, i8) {
        let mut x_move: i8 = 0;
        let mut y_move: i8 = 0;

        let e = (self.e as u32) + 1;
        let w = (self.w as u32) + 1;
        let n = (self.n as u32) + 1;
        let s = (self.s as u32) + 1;

        let total_proba = e + w + n + s;
        let dice = rand::thread_rng().gen_range(0, total_proba + 1);

        if dice < e {
            x_move = 1;
        } else if dice < e + w {
            x_move = -1;
        } else if dice < e + w + n {
            y_move = -1;
        } else {
            y_move = 1;
        }

        (x_move, y_move)
    }
}

#[derive(Clone)]
pub struct Creature {
    stats: CreatureStats,
    genes: Vec<Command>,
}
impl Creature {
    pub fn new(id: CreatureId, x: u32, y: u32, genes: Vec<Command>) -> Creature {
        Creature {
            stats: CreatureStats {
                id: id.clone(),
                pos_x: x,
                pos_y: y,
                energy: 100,
                e: 128,
                w: 128,
                n: 128,
                s: 128,
            },
            genes: genes,
        }
    }

    pub fn get_id(&self) -> CreatureId {
        self.stats.id.clone()
    }

    pub fn adjust_pos(
        world: &World,
        (pos_x, pos_y): (u32, u32),
        (dir_x, dir_y): (i8, i8),
    ) -> (u32, u32) {
        let mut ret_v: (i64, i64) = (pos_x as i64 + dir_x as i64, pos_y as i64 + dir_y as i64);
        let size_v = world.get_size();

        let check_boundaries = |v: i64, b: i64| -> i64 {
            if v < 0 {
                return b - 1;
            } else if v >= b {
                return v - b;
            }
            v
        };

        ret_v.0 = check_boundaries(ret_v.0, size_v.0 as i64);
        ret_v.1 = check_boundaries(ret_v.1, size_v.1 as i64);

        (ret_v.0 as u32, ret_v.1 as u32)
    }

    pub fn simulate(&mut self, world: &mut World, cmap: &mut CreatureMap) -> bool {
        if self.stats.energy == 0 {
            return false;
        }

        for g in &self.genes {
            g.execute(world, &mut self.stats, &self.genes, cmap);
        }

        let cur_tile = world.get_tile_mut(self.stats.pos_x, self.stats.pos_y);

        self.stats.energy -= 1;
        cur_tile.food += 1;

        if self.stats.energy == 0 {
            // Death by starvation
            cur_tile.creature = None;
            cmap.deallocate(self.stats.id.clone());
            false
        }
        else {
            true
        }
    }
}

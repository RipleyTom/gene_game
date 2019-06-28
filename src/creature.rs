pub mod command;

use crate::creaturemap::{CreatureId, CreatureMap};
use crate::world::World;
use command::Command;

use rand::Rng;

use std::fmt;

#[derive(Clone)]
pub enum CreatureType {
    Herbivore,
    Carnivore,
    Omnivore,
}
impl fmt::Display for CreatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CreatureType::Herbivore => "Herbivore",
                CreatureType::Carnivore => "Carnivore",
                CreatureType::Omnivore => "Omnivore",
            }
        )
    }
}


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
impl fmt::Display for CreatureStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Energy: {} E: {} W: {} N: {} S: {}",
            self.energy, self.e, self.w, self.n, self.s
        )
    }
}


#[derive(Clone)]
pub struct Creature {
    stats: CreatureStats,
    genes: Vec<Command>,
    ctype: CreatureType,
}
impl Creature {
    pub fn new(id: CreatureId, x: u32, y: u32, genes: Vec<Command>) -> Creature {
        let ctype = {
            let (mut eat_found, mut attack_found) = (false, false);
            for gene in &genes {
                if let Command::Eat = gene {
                    eat_found = true;
                } else if let Command::Attack = gene {
                    attack_found = true;
                }
            }

            if eat_found && attack_found {
                CreatureType::Omnivore
            } else if attack_found {
                CreatureType::Carnivore
            } else {
                CreatureType::Herbivore
            }
        };


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
            ctype: ctype,
        }
    }

    pub fn get_id(&self) -> CreatureId {
        self.stats.id.clone()
    }

    pub fn get_type(&self) -> CreatureType {
        self.ctype.clone()
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

        let energy_loss: u32 = match &self.ctype {
            CreatureType::Herbivore => 1,
            CreatureType::Carnivore => 5,
            CreatureType::Omnivore => 10,
        };

        if self.stats.energy <= energy_loss {
            cur_tile.food += self.stats.energy;
            self.stats.energy = 0;
            // Death by starvation
            cur_tile.creature = None;
            cmap.deallocate(self.stats.id.clone());
            false
        } else {
            cur_tile.food += energy_loss;
            self.stats.energy -= energy_loss;
            true
        }
    }
}
impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "==Creature==\n")?;

        write!(f, "Type: {}\n", self.ctype)?;
        write!(f, "Stats: {}\n", self.stats)?;
        write!(f, "Genes({}):\n", self.genes.len())?;

        for gene in &self.genes {
            write!(f, "{}\n", gene)?;
        }
        write!(f, "============")
    }
}


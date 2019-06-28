use crate::creaturemap::CreatureId;

use std::fmt;

pub struct World {
    width: u32,
    height: u32,
    map: Vec<Tile>,
}

#[derive(Clone)]
pub struct Tile {
    pub food: u32,
    pub creature: Option<CreatureId>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Food: {}", self.food)
    }
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            width: width,
            height: height,
            map: vec![
                Tile {
                    food: 100,
                    creature: None,
                };
                (width * height) as usize
            ],
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_tile(&self, x: u32, y: u32) -> &Tile {
        &self.map[((y * self.width) + x) as usize]
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> &mut Tile {
        &mut self.map[((y * self.width) + x) as usize]
    }

    pub fn get_num_creatures(&self) -> u32 {
        let mut nc = 0;
        for tile in &self.map {
            if let Some(_) = tile.creature {
                nc += 1;
            }
        }
        nc
    }
}

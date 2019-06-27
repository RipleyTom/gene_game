use crate::creaturemap::CreatureId;

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
        &self.map[((y * self.height) + x) as usize]
    }

    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> &mut Tile {
        &mut self.map[((y * self.height) + x) as usize]
    }
}

use crate::creature::command::Command;
use crate::creature::Creature;
use std::sync::atomic::{AtomicU64, Ordering};

static GENCOUNTER: AtomicU64 = AtomicU64::new(0);
pub fn get_generation() -> u64 {
    GENCOUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Eq, PartialEq, Clone)]
pub struct CreatureId {
    index: usize,
    generation: u64,
}
impl CreatureId {
    pub fn get_index(&self) -> usize {
        self.index
    }
}

pub struct CreatureMap {
    map: Vec<Option<Creature>>,
    free: Vec<usize>,
}
impl CreatureMap {
    pub fn new() -> CreatureMap {
        CreatureMap {
            map: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn get_num(&self) -> usize {
        self.map.len()
    }

    pub fn get_creatureid_by_index(&self, index: usize) -> Option<CreatureId> {
        if index >= self.map.len() {
            return None;
        }

        match &self.map[index] {
            Some(c) => Some(c.get_id()),
            None => None,
        }
    }

    fn allocate(&mut self) -> CreatureId {
        let generation = get_generation();
        let index;
        if self.free.len() != 0 {
            index = self.free.pop().unwrap();
        } else {
            index = self.map.len();
            self.map.push(None);
        }

        CreatureId {
            index: index,
            generation: generation,
        }
    }

    pub fn deallocate(&mut self, id: CreatureId) -> bool {
        if id.get_index() >= self.map.len() {
            return false;
        }

        match &self.map[id.get_index()] {
            Some(c) => {
                if c.get_id() == id {
                    self.free.push(id.get_index());
                    self.map[id.get_index()] = None;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    pub fn get_creature(&self, id: CreatureId) -> Option<&Creature> {
        if id.get_index() >= self.map.len() {
            return None;
        }
        match &self.map[id.get_index()] {
            Some(c) => {
                if c.get_id() == id {
                    return Some(c);
                }
                None
            }
            None => None,
        }
    }

    pub fn get_creature_mut(&mut self, id: CreatureId) -> Option<&mut Creature> {
        if id.get_index() >= self.map.len() {
            return None;
        }
        match &mut self.map[id.get_index()] {
            Some(c) => {
                if c.get_id() == id {
                    return Some(c);
                }
                None
            }
            None => None,
        }
    }

    pub fn set_creature(&mut self, id: CreatureId, creat: Creature) {
        if id.get_index() >= self.map.len() {
            return;
        }
        self.map[id.get_index()] = Some(creat);
    }

    pub fn add_creature(&mut self, x: u32, y: u32, gene: Vec<Command>) -> CreatureId {
        let id = self.allocate();
        self.set_creature(
            id.clone(),
            Creature::new(id.clone(), x, y, gene),
        );
        id
    }
}

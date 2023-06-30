use rand::prelude::*;
use std::collections::{HashMap, HashSet};

use rglk_math::vectors::{Vector2I, ORTHO_DIRECTIONS};
use::rglk_storage::{Entity, World};

use crate::components::{Fixture, Name, Position, Tile};

pub struct Board {
    pub tiles: HashMap<Vector2I, Entity>
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: HashMap::new() }
    }
    pub fn generate(&mut self, world: &mut World) {
        for x in 0..8 as i32 {
            for y in 0..8 as i32 {
                let v = Vector2I::new(x, y);
                let entity = world.spawn_entity();
                let _ = world.insert_component::<Name>(entity, Name("Tile".into()));
                let _ = world.insert_component::<Position>(entity, Position(v));
                let _ = world.insert_component::<Tile>(entity, Tile);
                self.tiles.insert(v, entity);
            }
        }
    }
}

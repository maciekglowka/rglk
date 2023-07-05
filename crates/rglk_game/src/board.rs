use rand::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

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
        let layout = BoardLayout::generate();
        for v in layout.tiles {
            let entity = world.spawn_entity();
            let _ = world.insert_component::<Name>(entity, Name("Tile".into()));
            let _ = world.insert_component::<Position>(entity, Position(v));
            let _ = world.insert_component::<Tile>(entity, Tile);
            self.tiles.insert(v, entity);
        }
    }
}

struct BoardLayout {
    pub tiles: HashSet<Vector2I>
}
impl BoardLayout {
    pub fn generate() -> Self {
        let mut tiles = HashSet::new();
        let mut rng = thread_rng();
        let max_dist = 20;
        let mut queue = VecDeque::new();
        queue.push_back(Vector2I::ZERO);

        while let Some(current) = queue.pop_front() {
            let neighbours = ORTHO_DIRECTIONS.iter()
                .map(|d| current + *d)
                .filter(|v| !queue.contains(v))
                .filter(|v| !tiles.contains(v))
                .filter(|v| v.x.abs() <= max_dist && v.y.abs() <= max_dist)
                .collect::<Vec<_>>();
            for n in neighbours {
                // if rng.gen_bool(0.2) { continue; }
                queue.push_back(n);
            }
            tiles.insert(current);
        }

        BoardLayout { tiles }
    }
}

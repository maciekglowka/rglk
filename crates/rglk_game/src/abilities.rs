use std::{
    collections::HashMap,
    f32::consts::PI
};

use rglk_math::vectors::{Vector2I, ORTHO_DIRECTIONS};
use rglk_storage::{Entity, World};

use super::actions::{Action, PlaceBouy, Shoot, Travel};
use super::board::Board;
use super::components::{Blocker, Position};
use super::wind::Wind;

pub trait Ability {
    fn get_possible_actions(
        &self,
        entity: Entity,
        world: &World
    ) -> HashMap<Vector2I, Box<dyn Action>>;
    fn description(&self) -> String;
}

pub struct Sailing;
impl Ability for Sailing {
    fn description(&self) -> String {
        "Sailing".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(wind) = world.get_resource::<Wind>() else { return output };
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let dist = match wind.current().angle(&dir) {
                a if (PI - 0.1..PI + 0.1).contains(&a) => continue,
                a if (-0.1..0.1).contains(&a) => 2,
                _ => 1
            };
            if let Some(target) = self.get_valid_tile(position.0, dir, dist, world) {
                output.insert(dir, Box::new(Travel { entity, target }));
            }
        }

        output
    }
}
impl Sailing {
    fn get_valid_tile(
        &self,
        source: Vector2I,
        dir: Vector2I,
        dist: i32,
        world: &World
    ) -> Option<Vector2I> {
        // gets a furthest possible valid tile at a sailing direction
        for d in (1..=dist).rev() {
            let v = source + dir * d;
            if is_tile_traversible(v, world) { return Some(v) }
        }
        None
    }
}

pub struct Swimming;
impl Ability for Swimming {
    fn description(&self) -> String {
        "Swimming".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let target = position.0 + dir;
            if is_tile_traversible(target, world) {
                output.insert(dir, Box::new(Travel { entity, target }));
            }
        }
        output.insert(Vector2I::ZERO, Box::new(Travel { entity, target: position.0 }));

        output
    }
}

pub struct Cannons {
    pub dist: u32,
    pub damage: u32
}
impl Ability for Cannons {
    fn description(&self) -> String {
        "Cannons".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            output.insert(dir, Box::new(Shoot {
                source: position.0,
                dir,
                dist: self.dist,
                damage: self.damage 
            }));
        }
        output
    }
}

pub struct Bouy {
    pub health: u32
}
impl Ability for Bouy {
    fn description(&self) -> String {
        "Buoy".into()
    }
    fn get_possible_actions(&self, entity: Entity, world: &World) -> HashMap<Vector2I, Box<dyn Action>> {
        let mut output = HashMap::new();
        let Some(position) = world.get_component::<Position>(entity) else { return output };

        for dir in ORTHO_DIRECTIONS {
            let target = position.0 + dir;
            if is_tile_traversible(target, world) {
                output.insert(dir, Box::new(PlaceBouy { position: target, health: self.health }));
            }
        }
        output
    }
}

fn is_tile_traversible(v: Vector2I, world: &World) -> bool {
    let Some(board) = world.get_resource::<Board>() else { return false };
    if !board.tiles.contains_key(&v) { return false }
    for item in world.query::<Position>().with::<Blocker>().iter() {
        if item.get::<Position>().unwrap().0 == v { return false };
    }
    true
}
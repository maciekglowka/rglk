use std::f32::consts::PI;

use rglk_math::vectors::Vector2I;
use rglk_storage::{Entity, World};

use super::Wind;
use super::board::Board;
use super::components::Position;

pub trait Action {
    fn execute(&self, world: &mut World) -> bool;
}

pub struct Sail {
    pub entity: Entity,
    pub direction: Vector2I
}
impl Action for Sail {
    fn execute(&self, world: &mut World) -> bool {
        let Some(mut positions) = world.get_component_set_mut::<Position>()
            else { return false };
        let Some(board) = world.get_resource::<Board>() else { return false };
        let Some(wind) = world.get_resource::<Wind>() else { return false };
        if let Some(position) = positions.get_mut(self.entity) {
            let offset = match wind.current().angle(&self.direction) {
                a if (PI - 0.1..PI + 0.1).contains(&a) => return false,
                a if (-0.1..0.1).contains(&a) => self.direction * 2,
                _ => self.direction
            };

            let new = position.0 + offset;
            if board.tiles.contains_key(&new) {
                position.0 = new;
                return true
            }
        }
        false
    }
}
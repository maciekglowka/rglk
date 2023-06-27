use rglk_math::vectors::Vector2I;
use rglk_storage::{Entity, World};

use super::components::Position;

pub trait Action {
    fn execute(&self, world: &mut World);
}

pub struct Walk {
    pub entity: Entity,
    pub direction: Vector2I
}
impl Action for Walk {
    fn execute(&self, world: &mut World) {
        let Some(mut positions) = world.get_component_set_mut::<Position>()
            else { return };
        if let Some(position) = positions.get_mut(self.entity) {
            position.0 += self.direction;
        }
    }
}
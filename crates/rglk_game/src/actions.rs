use rglk_math::vectors::Vector2I;
use rglk_storage::{Entity, World};

use std::collections::VecDeque;

use super::components::{Actor, Player, Position};

pub struct ActorQueue(pub VecDeque<Entity>);

pub trait Action {
    fn execute(&self, world: &mut World);
    fn score(&self, world: &World) -> i32 { 0 }
}

pub struct Travel {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Travel {
    fn execute(&self, world: &mut World) {
        let Some(mut position) = world.get_component_mut::<Position>(self.entity)
            else { return };
        position.0 = self.target;
    }
    fn score(&self, world: &World) -> i32 {
        let player_query = world.query::<Player>().with::<Position>();
        let Some(player) = player_query.iter().next() else { return 0 };
        let Some(pos) = player.get::<Position>() else { return 0 };

        20 - self.target.manhattan(pos.0)
    }
}

pub struct Pause;
impl Action for Pause {
    fn execute(&self, world: &mut World) {}
}
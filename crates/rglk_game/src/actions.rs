use rglk_math::vectors::Vector2I;
use rglk_storage::{Entity, World};

use std::collections::VecDeque;

use super::components::{Blocker, Health, Name, Player, Position, Projectile};

pub struct PendingActions(pub Vec<Box<dyn Action>>);
pub struct ActorQueue(pub VecDeque<Entity>);

pub trait Action {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>>;
    fn score(&self, world: &World) -> i32 { 0 }
}

pub struct Travel {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Travel {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut position = world.get_component_mut::<Position>(self.entity)?;
        position.0 = self.target;
        None
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<Player>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };

        20 - self.target.manhattan(player_position)
    }
}

pub struct Shoot {
    pub source: Vector2I,
    pub dir: Vector2I,
    pub dist: u32,
    pub damage:  u32
}
impl Shoot {
    fn get_target(&self, world: &World) -> Vector2I {
        let blocker_positions = world.query::<Blocker>().with::<Position>()
        .iter()
        .map(|i| i.get::<Position>().unwrap().0)
        .collect::<Vec<_>>();

        // find target - eg. the max dist or first blocker on the way
        let mut target = self.source;
        for _ in 1..=self.dist {
            target += self.dir;
            if blocker_positions.contains(&target) { break };
        }
        target
    }
}
impl Action for Shoot {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let target = self.get_target(world);
        let entity = world.spawn_entity();
        let _ = world.insert_component(entity, Projectile{
            damage: self.damage,
            target,
            source: self.source
        });
        None
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<Player>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };
        let target = self.get_target(world);
        if target == player_position {
            100
        } else {
            0
        }
    }
}

pub struct Pause;
impl Action for Pause {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> { None }
}

pub struct Damage {
    pub entity: Entity,
    pub value: u32
}
impl Action for Damage {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut health = world.get_component_mut::<Health>(self.entity)?;
        health.0 = health.0.saturating_sub(self.value);
        None
    }
    // score is not implemented as it always should be a resulting action
}
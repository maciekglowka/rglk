use rglk_math::vectors::Vector2I;
use rglk_storage::{Entity, World};

use std::collections::VecDeque;

use super::components::{Blocker, Health, Name, Player, PlayerCharacter, Position, Projectile};

pub struct PendingActions(pub Vec<Box<dyn Action>>);
pub struct ActorQueue(pub VecDeque<Entity>);

// data sharing struct (eg. for use with the graphics module)
// if this won't be generic enough, events would have to accept cloning
#[derive(Clone, Copy)]
pub struct ActionData {
    pub name: &'static str,
    pub entity: Option<Entity>,
    pub position: Option<Vector2I>,
    pub value: Option<i32>
}

pub trait Action {
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>>;
    fn score(&self, world: &World) -> i32 { 0 }
    fn as_data(&self) -> ActionData;
}

pub struct Travel {
    pub entity: Entity,
    pub target: Vector2I
}
impl Action for Travel {
    fn as_data(&self) -> ActionData {
        ActionData { name: "Travel", entity: Some(self.entity), position: Some(self.target), value: None }
    }
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut position = world.get_component_mut::<Position>(self.entity)?;
        position.0 = self.target;
        None
    }
    fn score(&self, world: &World) -> i32 {
        let Some(player_position) = world.query::<PlayerCharacter>().with::<Position>()
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
    fn as_data(&self) -> ActionData {
        ActionData { name: "Shoot", entity: None, position: Some(self.source), value: Some(self.damage as i32) }
    }
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
        let Some(player_position) = world.query::<PlayerCharacter>().with::<Position>()
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

pub struct PlaceBouy {
    pub position: Vector2I,
    pub health:  u32
}
impl Action for PlaceBouy {
    fn as_data(&self) -> ActionData {
        ActionData { name: "PlaceBuoy", entity: None, position: Some(self.position), value: Some(self.health as i32) }
    }
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let entity = world.spawn_entity();
        let _ = world.insert_component(entity, Name("Buoy".into()));
        let _ = world.insert_component(entity, Blocker);
        let _ = world.insert_component(entity, Position(self.position));
        let _ = world.insert_component(entity, Player);
        let _ = world.insert_component(entity, Health(self.health));
        None
    }
    fn score(&self, world: &World) -> i32 {
        // atm whatever ;)
        25
    }
}


pub struct Pause;
impl Action for Pause {
    fn as_data(&self) -> ActionData {
        ActionData { name: "Pause", entity: None, position: None, value: None }
    }
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> { None }
}

pub struct MeleeAttack {
    pub entity: Entity,
    pub target: Vector2I,
    pub damage: u32
}
impl Action for MeleeAttack {
    fn as_data(&self) -> ActionData {
        ActionData { name: "Melee", entity: Some(self.entity), position: Some(self.target), value: Some(self.damage as i32) }
    }
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut result = Vec::new();
        for item in world.query::<Health>().with::<Position>().iter() {
            if item.get::<Position>().unwrap().0 != self.target { continue }
            result.push(
                Box::new(Damage { entity: item.entity, value: self.damage })
                as Box<dyn Action>
            )
        }
        Some(result)
    }
    fn score(&self, world: &World) -> i32 {
        // note actually used
        let Some(player_position) = world.query::<PlayerCharacter>().with::<Position>()
            .iter()
            .map(|i| i.get::<Position>().unwrap().0)
            .next()
            else { return 0 };

        if player_position == self.target { return 200 }
        0
    }
}

pub struct Damage {
    pub entity: Entity,
    pub value: u32
}
impl Action for Damage {
    fn as_data(&self) -> ActionData {
        ActionData { name: "Damage", entity: Some(self.entity), position: None, value: Some(self.value as i32) }
    }
    fn execute(&self, world: &mut World) -> Option<Vec<Box<dyn Action>>> {
        let mut health = world.get_component_mut::<Health>(self.entity)?;
        health.0 = health.0.saturating_sub(self.value);
        None
    }
    // score is not implemented as it always should be a resulting action
}
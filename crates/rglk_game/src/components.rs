use::rglk_storage::{Component, Entity};

use rglk_math::vectors::Vector2I;

use super::abilities::Ability;
use super::actions::Action;

pub struct Actor {
    pub cards: Vec<Entity>,
    pub action: Option<Box<dyn Action>>
}
impl Component for Actor {}

// actor cannot travel to a blocked tile
pub struct Blocker;
impl Component for Blocker {}

pub struct Card(pub Box<dyn Ability>);
impl Component for Card {
    fn as_str(&self) -> String {
        self.0.description()
    }
}

// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

pub struct Health(pub u32);
impl Component for Health {}

pub struct Melee(pub u32);
impl Component for Melee {}

pub struct Name (pub String);
impl Component for Name {}

// many can exist in the world
// marks entities 'allied' or spawned by the player

pub struct Player;
impl Component for Player {}

// only on in the game world
// the actual player
pub struct PlayerCharacter {
    pub active_card: usize
}
impl Component for PlayerCharacter {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Projectile{
    pub damage: u32,
    pub source: Vector2I,
    pub target: Vector2I
}
impl Component for Projectile {}

pub struct Tile;
impl Component for Tile {}
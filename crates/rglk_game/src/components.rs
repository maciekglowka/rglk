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
impl Component for Card {}

// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

pub struct Name (pub String);
impl Component for Name {}

pub struct Player {
    pub active_card: usize
}
impl Component for Player {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}
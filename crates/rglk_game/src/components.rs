use::rglk_storage::{Component, Entity};

use rglk_math::vectors::Vector2I;

use super::actions::Action;

pub struct Actor {
    pub next: Option<Box<dyn Action>>
}
impl Component for Actor {}

// fixed tile furnishings
pub struct Fixture;
impl Component for Fixture {}

pub struct Name (pub String);
impl Component for Name {}

pub struct Player;
impl Component for Player {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}
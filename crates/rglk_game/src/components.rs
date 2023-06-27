use::rglk_storage::Component;

use rglk_math::vectors::Vector2I;

use super::actions::Action;

pub struct Actor {
    pub next: Option<Box<dyn Action>>
}
impl Component for Actor {}

pub struct Piece;
impl Component for Piece {}

pub struct Player;
impl Component for Player {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}
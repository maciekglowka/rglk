use::rglk_storage::Component;

use crate::vectors::Vector2Int;

pub struct Position(pub Vector2Int);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}
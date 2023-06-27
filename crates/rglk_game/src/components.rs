use::rglk_storage::Component;

use rglk_math::vectors::Vector2I;

pub struct Piece;
impl Component for Piece {}

pub struct Position(pub Vector2I);
impl Component for Position {}

pub struct Tile;
impl Component for Tile {}
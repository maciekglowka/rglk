use std::collections::HashMap;

use rglk_math::vectors::Vector2I;
use::rglk_storage::Entity;

pub struct Board {
    pub tiles: HashMap<Vector2I, Entity>
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: HashMap::new() }
    }
}


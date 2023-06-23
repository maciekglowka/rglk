use std::collections::HashMap;

use::rglk_storage::Entity;

use crate::vectors::Vector2Int;
pub struct Board {
    pub tiles: HashMap<Vector2Int, Entity>
}
impl Board {
    pub fn new() -> Self {
        Board { tiles: HashMap::new() }
    }
}


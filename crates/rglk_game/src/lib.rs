use rglk_math::vectors::Vector2I;
use::rglk_storage::World;

mod board;
pub mod components;

pub use board::Board;

pub fn game_loop(world: &mut World) {

}

pub fn init(world: &mut World) {
    let mut board = board::Board::new();

    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2I::new(x, y);
            let entity = world.spawn_entity();
            let _ = world.insert_component::<components::Position>(entity, components::Position(v));
            let _ = world.insert_component::<components::Tile>(entity, components::Tile);
            board.tiles.insert(v, entity);
        }
    }

    world.insert_resource::<Board>(board);
}

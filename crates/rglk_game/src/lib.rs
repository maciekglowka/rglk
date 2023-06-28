use rglk_math::vectors::Vector2I;
use::rglk_storage::World;

pub mod actions;
mod board;
pub mod globals;
pub mod components;

pub use board::Board;

pub fn game_step(world: &mut World) {
    let query = world.query::<components::Player>().with::<components::Actor>();
    let action = match query.iter().next() {
        Some(item) => item.get_mut::<components::Actor>().unwrap().next.take(),
        None => return
    };
    if let Some(action) = action {
        action.execute(world);
    }
}

pub fn init(world: &mut World) {
    let board = board::generate_board(world);

    world.insert_resource::<Board>(board);

    let npc = world.spawn_entity();
    let _ = world.insert_component::<components::Position>(npc, components::Position(Vector2I::new(0, 0)));
    let _ = world.insert_component::<components::Piece>(npc, components::Piece);
    let _ = world.insert_component::<components::Player>(npc, components::Player);
    let _ = world.insert_component::<components::Actor>(npc, components::Actor { next: None });
}

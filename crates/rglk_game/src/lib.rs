use rglk_math::vectors::Vector2I;
use::rglk_storage::World;

pub mod actions;
mod board;
pub mod components;

pub use board::Board;

pub fn game_step(world: &mut World) {
    // only player for now
    let Some(players) = world.get_component_set::<components::Player>()
        else { return };
    let Some(player_entity) = players.one_entity() else { return };
    let Some(mut actors) = world.get_component_set_mut::<components::Actor>()
        else { return };
    let Some(actor) = actors.get_mut(*player_entity) else { return };
    let action = match actor.next.take() {
        Some(a) => a,
        _ => return
    };
    drop(players);
    drop(actors);
    action.execute(world);
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

    let npc = world.spawn_entity();
    let _ = world.insert_component::<components::Position>(npc, components::Position(Vector2I::new(0, 0)));
    let _ = world.insert_component::<components::Piece>(npc, components::Piece);
    let _ = world.insert_component::<components::Player>(npc, components::Player);
    let _ = world.insert_component::<components::Actor>(npc, components::Actor { next: None });
}

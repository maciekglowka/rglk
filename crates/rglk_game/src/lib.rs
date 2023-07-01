use std::collections::VecDeque;

use rglk_math::vectors::Vector2I;
use::rglk_storage::World;

pub mod actions;
pub mod abilities;
mod board;
pub mod globals;
pub mod components;
mod systems;
mod wind;

pub use board::Board;
pub use wind::Wind;
pub use systems::game_step;

pub fn init(world: &mut World) {
    let mut board = board::Board::new();
    board.generate(world);
    world.insert_resource(board);

    let wind = wind::Wind::new();
    world.insert_resource(wind);

    let queue = actions::ActorQueue(VecDeque::new());
    world.insert_resource(queue);

    let card = world.spawn_entity();
    let _ = world.insert_component(card, components::Card(
        Box::new(abilities::Sailing)
    ));

    let player = world.spawn_entity();
    let _ = world.insert_component(player, components::Position(Vector2I::new(0, 0)));
    let _ = world.insert_component(player, components::Name("Player".into()));
    let _ = world.insert_component(player, components::Player{
        active_card: 0
    });
    let _ = world.insert_component(player, components::Actor { 
        cards: vec![card],
        action: None
    });

    let rowers_card = world.spawn_entity();
    let _ = world.insert_component(rowers_card, components::Card(
        Box::new(abilities::Rowing)
    ));

    let npc = world.spawn_entity();
    let _ = world.insert_component(npc, components::Position(Vector2I::new(5, 5)));
    let _ = world.insert_component(npc, components::Name("Rowers".into()));
    let _ = world.insert_component(npc, components::Actor { 
        cards: vec![rowers_card],
        action: None
    });
}

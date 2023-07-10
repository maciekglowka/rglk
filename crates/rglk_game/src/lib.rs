use rand::prelude::*;
use std::collections::VecDeque;

use rglk_events::EventBus;
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

pub fn init(world: &mut World, manager: GameManager) {
    world.insert_resource(manager);

    let mut board = board::Board::new();
    board.generate(world);
    world.insert_resource(board);

    let wind = wind::Wind::new();
    world.insert_resource(wind);

    let queue = actions::ActorQueue(VecDeque::new());
    world.insert_resource(queue);

    let pending = actions::PendingActions(Vec::new());
    world.insert_resource(pending);

    let sail_card = world.spawn_entity();
    let _ = world.insert_component(sail_card, components::Card(
        Box::new(abilities::Sailing)
    ));
    let cannons_card = world.spawn_entity();
    let _ = world.insert_component(cannons_card, components::Card(
        Box::new(abilities::Cannons { dist: 4, damage: 2 })
    ));
    let buoy_card = world.spawn_entity();
    let _ = world.insert_component(buoy_card, components::Card(
        Box::new(abilities::Buoy { health: 2 })
    ));

    let player = world.spawn_entity();
    let _ = world.insert_component(player, components::Position(Vector2I::new(0, 0)));
    let _ = world.insert_component(player, components::Name("Player".into()));
    let _ = world.insert_component(player, components::Blocker);
    let _ = world.insert_component(player, components::Health(1));
    let _ = world.insert_component(player, components::Player);
    let _ = world.insert_component(player, components::PlayerCharacter{
        active_card: 0
    });
    let _ = world.insert_component(player, components::Actor { 
        cards: vec![sail_card, cannons_card, buoy_card],
        action: None
    });

    let rowers_card = world.spawn_entity();
    let _ = world.insert_component(rowers_card, components::Card(
        Box::new(abilities::Swimming)
    ));

    let mut rng = thread_rng();
    for _ in 0..3 {
        let v = Vector2I::new(
            rng.gen_range(4..8),
            rng.gen_range(4..8),
        );
        let npc = world.spawn_entity();
        let _ = world.insert_component(npc, components::Position(v));
        let _ = world.insert_component(npc, components::Name("Rowers".into()));
        let _ = world.insert_component(npc, components::Health(1));
        let _ = world.insert_component(npc, components::Melee(1));
        let _ = world.insert_component(npc, components::Blocker);
        let _ = world.insert_component(npc, components::Actor { 
            cards: vec![rowers_card],
            action: None
        });
    }
}

pub struct GameManager {
    pub action_events: EventBus<ActionEvent>
}
impl GameManager {
    pub fn new() -> Self {
        GameManager { action_events: EventBus::new() }
    }
}

#[derive(Clone, Copy)]
pub struct ActionEvent(pub actions::ActionData);
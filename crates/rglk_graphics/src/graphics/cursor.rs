use rglk_game::{
    actions::ActorQueue,
    components::{Actor, Card, PlayerCharacter, Position}
};
use rglk_math::vectors::Vector2F;
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use crate::SpriteColor;
use crate::globals::TILE_SIZE;

use super::{GraphicsBackend, GraphicsState};

pub fn draw_cursor(
    world: &World,
    state: &GraphicsState,
    backend: &dyn GraphicsBackend
) {
    let Some(queue) = world.get_resource::<ActorQueue>() else { return };
    let query = world.query::<PlayerCharacter>().with::<Position>();
    let Some(item) = query.iter().next() else { return };
    if queue.0.front() != Some(&item.entity) { return };
    let Some(actor) = item.get::<Actor>() else { return };
    let player = item.get::<PlayerCharacter>().unwrap();
    let Some(card) = world.get_component::<Card>(actor.cards[player.active_card]) else { return };
    for action in card.0.get_possible_actions(item.entity, world) {
        backend.draw_world_sprite(
            "ascii",
            249,
            action.0.as_f32() * TILE_SIZE,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            SpriteColor(255, 255, 255, 255)
        );
    }

}
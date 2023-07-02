use rglk_game::components::Position;
use rglk_storage::World;

use super::GraphicsState;

pub mod renderers;
mod utils;

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState
) -> bool {
    renderers::handle_world_events(world, state);
    let mut ready = renderers::update_sprites(
        &world.get_component_set::<Position>().unwrap(),
        state
    );
    ready = ready && renderers::update_projectiles(
        &world,
        state
    );
    renderers::draw_sprites(state);
    ready
}
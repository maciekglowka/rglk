use rogalik::storage::World;

use rglk_game::components::Position;

use super::GraphicsState;

mod cursor;
pub mod renderers;
pub mod utils;

use super::GraphicsBackend;

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState,
    backend: &dyn GraphicsBackend
) -> bool {
    renderers::handle_world_events(world, state);
    renderers::handle_action_events(world, state);
    let ready = renderers::update_sprites(state);
    // ready = ready && renderers::update_projectiles(
    //     &world,
    //     state
    // );
    renderers::draw_sprites(state, backend);
    cursor::draw_cursor(world, state, backend);
    ready
}
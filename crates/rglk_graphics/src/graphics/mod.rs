use rglk_game::components::Position;
use rglk_storage::World;

use super::GraphicsState;

pub mod renderers;
mod utils;

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState
) -> bool {
    renderers::spawn_sprites(world, state);
    let ready = renderers::update_sprites(
        &world.get_component_set::<Position>().unwrap(),
        state
    );
    renderers::draw_sprites(state);
    ready
}
use rglk_game::Wind;
use rglk_math::vectors::{Vector2I, Vector2F};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};
use rglk_sprites::{Assets, SpriteColor};

use super::GraphicsState;

pub fn ui_update(
    world: &World,
    state: &mut GraphicsState   
) {
    draw_wind_queue(world, state);
}


fn draw_wind_queue(world: &World, state: &GraphicsState) {
    let Some(wind) = world.get_resource::<Wind>() else { return };
    let Some(atlas) = state.assets.atlases.get("ascii") else { return };
    for (i, dir) in wind.queue.iter().enumerate() {
        let index = match *dir {
            Vector2I::DOWN => 31,
            Vector2I::UP => 30,
            Vector2I::LEFT => 17,
            Vector2I::RIGHT => 16,
            _ => continue
        };
        atlas.draw_sprite(
            Vector2F::new(32. * i as f32, 0.),
            Vector2F::new(32., 32.),
            index,
            SpriteColor(255, 255, 255, 255)
        );
    }
}
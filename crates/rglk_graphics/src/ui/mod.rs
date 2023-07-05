use rglk_game::Wind;
use rglk_math::vectors::{Vector2I, Vector2F};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

pub fn ui_update(
    world: &World,
    state: &mut GraphicsState,
    backend: &dyn GraphicsBackend
) {
    draw_wind_queue(world, backend);
}


fn draw_wind_queue(world: &World, backend: &dyn GraphicsBackend) {
    let Some(wind) = world.get_resource::<Wind>() else { return };
    for (i, dir) in wind.queue.iter().enumerate() {
        let index = match *dir {
            Vector2I::DOWN => 31,
            Vector2I::UP => 30,
            Vector2I::LEFT => 17,
            Vector2I::RIGHT => 16,
            _ => continue
        };
        backend.draw_ui_sprite(
            "ascii",
            index,
            Vector2F::new(32. * i as f32, 0.),
            Vector2F::new(32., 32.),
            SpriteColor(255, 255, 255, 255)
        );
    }
}
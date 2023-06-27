use macroquad::prelude::*;

use rglk_game;
use rglk_graphics;
use rglk_sprites;
use rglk_storage;

#[macroquad::main("RGLK")]
async fn main() {
    let mut graphics_assets = rglk_sprites::Assets::new();
    rglk_graphics::assets::load_assets(&mut graphics_assets).await;

    let mut world = rglk_storage::World::new();
    set_camera(&Camera2D {
        zoom: Vec2::new(2. / screen_width(), -2. / screen_height()),
        ..Default::default()
    });
    let mut graphics_state = rglk_graphics::GraphicsState::new(
        &mut world,
        graphics_assets
    );
    rglk_game::init(&mut world);

    loop {
        clear_background(BLACK);
        rglk_graphics::graphics_update(&world, &mut graphics_state);
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

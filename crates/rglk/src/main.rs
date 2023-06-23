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
    rglk_game::init(&mut world);

    
    loop {
        clear_background(BLACK);
        set_camera(&Camera2D {
            zoom: vec2(0.001, 0.001),
            target: vec2(0.0, 0.0),
            ..Default::default()
        });
        rglk_graphics::board::draw_board(&world, &graphics_assets);
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

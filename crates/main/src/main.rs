use macroquad::prelude::*;

use rglk_sprites;

#[macroquad::main("RGLK")]
async fn main() {
    // let tileset = load_texture("assets/sprites/ascii.png").await.unwrap();
    // tileset.set_filter(FilterMode::Nearest);

    let atlas = rglk_sprites::SpriteAtlas::new(
        "assets/sprites/ascii.png",
        16, 16,
        None
    ).await.unwrap();

    loop {
        clear_background(BLACK);

        atlas.draw_sprite(
            Vec2::splat(100.),
            Vec2::splat(64.),
            1,
            WHITE
        );
        atlas.draw_sprite(
            Vec2::splat(120.),
            Vec2::splat(64.),
            17,
            YELLOW
        );
        next_frame().await
    }
}

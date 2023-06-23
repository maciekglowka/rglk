use rglk_game::components::{Position, Tile};
use rglk_sprites::{Assets, SpriteColor};
use rglk_storage::{EntityFilter, World};

use crate::globals::TILE_SIZE;

pub fn draw_board(world: &World, assets: &Assets) {
    let positions = world.get_component_set::<Position>().unwrap();
    let tiles = world.get_component_set::<Tile>().unwrap();
    let entities = EntityFilter::from(tiles.entities())
        .combine(positions.entities());

    let atlas = assets.atlases.get("ascii").unwrap();

    for entity in entities {
        let p = positions.get(entity).unwrap();
        atlas.draw_sprite(
            (p.0.x as f32 * TILE_SIZE, p.0.y as f32 * TILE_SIZE),
            (TILE_SIZE, TILE_SIZE),
            177,
            SpriteColor(128, 128, 128, 255)
        );
    }
}
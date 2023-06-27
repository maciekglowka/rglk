use std::any::TypeId;

use rglk_game::components::{Piece, Position, Tile};
use rglk_math::vectors::Vector2F;
use rglk_sprites::{Assets, SpriteColor};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use super::GraphicsState;
use crate::globals::TILE_SIZE;

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: u32,
    pub color: SpriteColor
}

pub fn spawn_sprites(
    positions: &ComponentSet<Position>,
    pieces: &ComponentSet<Piece>,
    tiles: &ComponentSet<Tile>,
    state: &mut GraphicsState
) {
    let mut updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentSpawned(entity, type_id) => {
                if *type_id != TypeId::of::<Position>() {
                    continue;
                }
                let p = positions.get(*entity).unwrap();

                state.sprites.push(
                    get_sprite_renderer(*entity, p, pieces, tiles)
                );
                updated = true;
            },
            _ => continue
        }
    }
    if updated {
        state.sort_sprites();
    }
}

pub fn draw_sprites(state: &GraphicsState) {
    for sprite in state.sprites.iter() {
        let Some(atlas) = state.assets.atlases.get(&sprite.atlas_name) else { continue };
        atlas.draw_sprite(
            sprite.v,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            sprite.index, 
            sprite.color.into()
        );
    }
}

fn get_sprite_renderer(
    entity: Entity,
    position: &Position,
    pieces: &ComponentSet<Piece>,
    tiles: &ComponentSet<Tile>,
) -> SpriteRenderer {
    let mut index = 0;
    let mut z_index = 0;
    let mut color = SpriteColor(0, 0, 0, 0);
    if tiles.get(entity).is_some() {
        index = 177;
        z_index = 10;
        color = SpriteColor(100, 100, 100, 255);
    } else if pieces.get(entity).is_some() {
        index = 2;
        z_index = 100;
        color = SpriteColor(255, 0, 255, 255);
    }
    SpriteRenderer { 
        entity: entity,
        v: position.0.as_f32() * TILE_SIZE,
        atlas_name: "ascii".into(),
        index,
        z_index,
        color
    }
}
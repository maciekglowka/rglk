use std::any::TypeId;

use rglk_game::components::{Position, Tile};
use rglk_math::vectors::Vector2F;
use rglk_sprites::{Assets, SpriteColor};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use super::GraphicsState;
use crate::globals::TILE_SIZE;

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub atlas_name: String,
    pub index: u32
}

pub fn spawn_sprites(
    positions: &ComponentSet<Position>,
    state: &mut GraphicsState
) {
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentSpawned(entity, type_id) => {
                if *type_id != TypeId::of::<Position>() {
                    continue;
                }
                let p = positions.get(*entity).unwrap();

                state.sprites.push(
                    SpriteRenderer { 
                        entity: *entity,
                        v: p.0.as_f32() * TILE_SIZE,
                        atlas_name: "ascii".into(),
                        index: 177
                    }
                );
            },
            _ => continue
        }
    }
}

pub fn draw_sprites(state: &GraphicsState) {
    for sprite in state.sprites.iter() {
        let Some(atlas) = state.assets.atlases.get(&sprite.atlas_name) else { continue };
        atlas.draw_sprite(
            sprite.v,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            sprite.index, 
            SpriteColor(255, 255, 255, 255)
        );
    }
}
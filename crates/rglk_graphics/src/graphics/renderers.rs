use std::any::TypeId;

use rglk_game::components::{Actor, Fixture, Name, Position, Tile};
use rglk_math::vectors::Vector2F;
use rglk_sprites::{Assets, SpriteColor};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use super::super::GraphicsState;
use super::utils::move_towards;
use crate::globals::{TILE_SIZE, ACTOR_Z, FIXTURE_Z, TILE_Z, MOVEMENT_SPEED};

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: u32,
    pub color: SpriteColor
}

pub fn spawn_sprites(
    world: &World,
    state: &mut GraphicsState
) {
    let mut updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentSpawned(entity, type_id) => {
                if *type_id != TypeId::of::<Position>() {
                    continue;
                }
                state.sprites.push(
                    get_sprite_renderer(*entity, world)
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

pub fn update_sprites(
    positions: &ComponentSet<Position>,
    state: &mut GraphicsState
) -> bool {
    let mut ready = true;
    for sprite in state.sprites.iter_mut() {
        let Some(position) = positions.get(sprite.entity) else { continue };
        let target = position.0.as_f32() * TILE_SIZE;
        sprite.v = move_towards(sprite.v, target, MOVEMENT_SPEED);
        if sprite.v != target { ready = false }
    }
    ready
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
    world: &World,
) -> SpriteRenderer {
    let mut z_index = 0;

    let position = world.get_component::<Position>(entity).unwrap();
    let name = world.get_component::<Name>(entity).unwrap();

    if world.get_component::<Fixture>(entity).is_some() {
        z_index = FIXTURE_Z
    } else if world.get_component::<Tile>(entity).is_some() {
        z_index = TILE_Z
    } else if world.get_component::<Actor>(entity).is_some() {
        z_index = ACTOR_Z
    }

    let index = match name.0.as_str() {
        "Player" => 127,
        "Rowers" => 15,
        "Tile" => 177,
        _ => 0
    };
    let color = match name.0.as_str() {
        "Player" => SpriteColor(255, 255, 255, 255),
        "Rowers" => SpriteColor(255, 0, 255, 255),
        "Tile" => SpriteColor(50, 50, 200, 255),
        _ => SpriteColor(0, 0, 0, 0) 
    };

    SpriteRenderer { 
        entity: entity,
        v: position.0.as_f32() * TILE_SIZE,
        atlas_name: "ascii".into(),
        index,
        z_index,
        color
    }
}
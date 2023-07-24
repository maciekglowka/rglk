use std::{
    any::TypeId,
    collections::VecDeque
};
use rogalik::math::vectors::Vector2F;
use rogalik::storage::{ComponentSet, Entity, World, WorldEvent};

use rglk_game::components::{Actor, Fixture, Name, Position, Projectile, Tile};

use super::super::{GraphicsState, GraphicsBackend, SpriteColor};
use super::utils::move_towards;
use crate::globals::{TILE_SIZE, ACTOR_Z, FIXTURE_Z, PROJECTILE_Z, TILE_Z, MOVEMENT_SPEED};

pub struct SpriteRenderer {
    pub entity: Entity,
    pub v: Vector2F,
    pub path: VecDeque<Vector2F>,
    pub atlas_name: String,
    pub index: u32,
    pub z_index: u32,
    pub color: SpriteColor
}

pub fn handle_world_events(
    world: &World,
    state: &mut GraphicsState
) {
    let mut sprites_updated = false;
    for ev in state.ev_world.read().iter().flatten() {
        match ev {
            WorldEvent::ComponentSpawned(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Position>() => {
                        state.sprites.push(
                            get_sprite_renderer(*entity, world)
                        );
                        sprites_updated = true;
                    },
                    a if a == TypeId::of::<Projectile>() => {
                        state.sprites.push(
                            get_projectile_renderer(*entity, world)
                        );
                        sprites_updated = true;
                    },
                    _ => continue
                }
            },
            WorldEvent::ComponentRemoved(entity, type_id) => {
                match *type_id {
                    a if a == TypeId::of::<Position>() || a == TypeId::of::<Projectile>() => {
                        state.sprites.retain(|a| a.entity != *entity);
                    },
                    _ => continue
                }
            }
            _ => continue
        }
    }
    if sprites_updated {
        state.sort_sprites();
    }
}

pub fn handle_action_events(
    world: &World,
    state: &mut GraphicsState
) {
    for ev in state.ev_actions.read().iter().flatten() {
        match ev.0.name {
            "Melee" => {
                if let Some(sprite) = get_entity_sprite(ev.0.entity.unwrap(), state) {
                    sprite.path.push_back(ev.0.position.unwrap().as_f32() * TILE_SIZE);
                    sprite.path.push_back(sprite.v);
                }
            },
            "Travel" => {
                if let Some(sprite) = get_entity_sprite(ev.0.entity.unwrap(), state) {
                    sprite.path.push_back(ev.0.position.unwrap().as_f32() * TILE_SIZE);
                }
            },
            _ => continue
        }
    }
}

pub fn update_sprites(
    state: &mut GraphicsState
) -> bool {
    let mut ready = true;
    for sprite in state.sprites.iter_mut() {
        let Some(target) = sprite.path.get(0) else { continue };
        sprite.v = move_towards(sprite.v, *target, MOVEMENT_SPEED);
        if sprite.v == *target {
            sprite.path.pop_front();
        }
        if sprite.path.len() > 0 { ready = false }
    }
    ready
}

pub fn draw_sprites(state: &GraphicsState, backend: &dyn GraphicsBackend) {
    for sprite in state.sprites.iter() {
        backend.draw_world_sprite(
            &sprite.atlas_name,
            sprite.index,
            sprite.v,
            Vector2F::new(TILE_SIZE, TILE_SIZE),
            sprite.color
        );
    }
}

fn get_sprite_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let mut z_index = 0;

    let name = world.get_component::<Name>(entity).unwrap();
    let position = world.get_component::<Position>(entity).unwrap();

    if world.get_component::<Fixture>(entity).is_some() {
        z_index = FIXTURE_Z
    } else if world.get_component::<Tile>(entity).is_some() {
        z_index = TILE_Z
    } else if world.get_component::<Actor>(entity).is_some() {
        z_index = ACTOR_Z
    }

    let index = match name.0.as_str() {
        "Buoy" => 9,
        "Player" => 127,
        "Rowers" => 15,
        "Tile" => 177,
        _ => 0
    };
    let color = match name.0.as_str() {
        "Buoy" => SpriteColor(255, 255, 0, 255),
        "Player" => SpriteColor(255, 255, 255, 255),
        "Rowers" => SpriteColor(255, 0, 255, 255),
        "Tile" => SpriteColor(50, 50, 200, 255),
        _ => SpriteColor(0, 0, 0, 0) 
    };

    SpriteRenderer { 
        entity: entity,
        v: position.0.as_f32() * TILE_SIZE,
        path: VecDeque::new(),
        atlas_name: "ascii".into(),
        index,
        z_index,
        color
    }
}

fn get_projectile_renderer(
    entity: Entity,
    world: &World,
) -> SpriteRenderer {
    let projectile = world.get_component::<Projectile>(entity).unwrap();
    let mut path = VecDeque::new();
    path.push_back(projectile.target.as_f32() * TILE_SIZE);

    SpriteRenderer { 
        entity: entity,
        v: projectile.source.as_f32() * TILE_SIZE,
        path,
        atlas_name: "ascii".into(),
        index: 249,
        z_index: PROJECTILE_Z,
        color: SpriteColor(255, 255, 255, 255)
    }
}

fn get_entity_sprite(entity: Entity, state: &mut GraphicsState) -> Option<&mut SpriteRenderer> {
    state.sprites.iter_mut()
        .find(|a| a.entity == entity)
}
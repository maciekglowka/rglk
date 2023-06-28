use rand::prelude::*;
use std::collections::HashMap;

use rglk_math::vectors::Vector2I;
use::rglk_storage::{Entity, World};

use crate::components::{Position, Tile};
use crate::globals::{CHUNK_SIZE, GRID_SIZE};

pub struct Board {
    pub chunks: HashMap<Vector2I, Chunk>,
    pub current_chunk: Vector2I
}
impl Board {
    pub fn new() -> Self {
        Board { chunks: HashMap::new(), current_chunk: Vector2I::ZERO }
    }
    pub fn get_current_chunk(&self) -> &Chunk {
        &self.chunks[&self.current_chunk]
    }
}

pub struct Chunk {
    position: Vector2I,
    pub tiles: HashMap<Vector2I, Entity>
}
impl Chunk {
    pub fn new(position: Vector2I) -> Self {
        Chunk { position, tiles: HashMap::new() }
    }
}

pub fn generate_board(world: &mut World) -> Board {
    let mut board = Board::new();
    for x in 0..GRID_SIZE as i32 {
        for y in 0..GRID_SIZE as i32 {
            let v = Vector2I::new(x, y);
            let chunk = generate_chunk(world, v);
            board.chunks.insert(v, chunk);
        }
    }
    board
}

fn generate_chunk(world: &mut World, chunk_v: Vector2I) -> Chunk {
    let mut chunk = Chunk::new(chunk_v);
    let mut rng = thread_rng();
    let offset = chunk_v * (CHUNK_SIZE + 1) as i32;

    for x in 0..CHUNK_SIZE as i32 {
        for y in 0..CHUNK_SIZE as i32 {
            if rng.gen_bool(0.25) { continue }
            let v = offset + Vector2I::new(x, y);
            let entity = world.spawn_entity();
            let _ = world.insert_component::<Position>(entity, Position(v));
            let _ = world.insert_component::<Tile>(entity, Tile);
            chunk.tiles.insert(v, entity);
        }
    }

    if chunk_v.y != 0 {
        let v = offset + Vector2I::new(2, -1);
        let entity = world.spawn_entity();
        let _ = world.insert_component::<Position>(entity, Position(v));
        let _ = world.insert_component::<Tile>(entity, Tile);
        chunk.tiles.insert(v, entity);
    }

    chunk
}

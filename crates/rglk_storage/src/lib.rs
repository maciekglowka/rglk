mod errors;
mod sparse;
mod world;

pub use world::World;

type IdSize = u16;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Entity {
    pub id: IdSize,
    pub version: IdSize
}


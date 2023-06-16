// use std::any::{Any, TypeId};

mod errors;
mod sparse;

type IdSize = u16;

pub struct Entity {
    pub id: IdSize,
    pub version: IdSize
}


use std::any::Any;

mod component;
mod component_storage;
mod entity;
mod errors;
mod events;
mod resource;
mod world;

pub use component::Component;
pub use entity::{Entity, EntityFilter};
pub use events::{WorldEvent};
pub use component_storage::ComponentSet;
pub use world::World;


pub trait Storage {
    fn as_any(&self) -> &dyn Any;
    fn get_as_component(&self, entity: Entity) -> Option<Box<&dyn Component>>;
    fn remove_untyped(&self, entity: Entity) -> Option<Box<dyn Component>>;
}
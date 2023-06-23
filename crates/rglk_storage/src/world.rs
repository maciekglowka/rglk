use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use super::component::Component;
use super::entity::Entity;
use super::errors::EntityError;
use super::sparse::{ComponentSet, StorageCell, Storage};

pub struct World {
    component_storage: HashMap<TypeId, Box<dyn Storage>>,
}
impl World {
    pub fn new() -> Self {
        World { component_storage: HashMap::new() }
    }
    pub fn get_component_set<T: Component + 'static>(&self) -> Option<Ref<ComponentSet<T>>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &StorageCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow())
    }
    pub fn get_component_set_mut<T: Component + 'static>(&self) -> Option<RefMut<ComponentSet<T>>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &StorageCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow_mut())
    }
    fn insert_component_storage<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let set = ComponentSet::<T>::new();
        let storage = StorageCell { inner: RefCell::new(set) };
        self.component_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
    pub fn insert_component<T: Component + 'static>(&mut self, entity: Entity, component: T) -> Result<(), EntityError> {
        let type_id = TypeId::of::<T>();
        if !self.component_storage.contains_key(&type_id) {
            self.insert_component_storage::<T>()
        }
        self.get_component_set_mut()
            .ok_or(EntityError)?.insert(entity, component)
    }
}

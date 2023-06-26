use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use rglk_events::EventBus;

use super::{Storage, WorldEvent};
use super::component::Component;
use super::component_storage::{ComponentSet, ComponentCell, ComponentStorage};
use super::entity::{Entity, EntityStorage};
use super::errors::EntityError;
use super::resource::ResourceCell;

pub struct World {
    component_storage: HashMap<TypeId, Box<dyn ComponentStorage>>,
    entitiy_storage: EntityStorage,
    resource_storage: HashMap<TypeId, Box<dyn Storage>>,
    pub events: EventBus<WorldEvent>
}
impl World {
    pub fn new() -> Self {
        World { 
            component_storage: HashMap::new(),
            resource_storage: HashMap::new(),
            entitiy_storage: EntityStorage::new(),
            events: EventBus::new()
        }
    }

    // entities

    pub fn spawn_entity(&mut self) -> Entity {
        self.entitiy_storage.spawn()
    }
    pub fn despawn_entity(&mut self, entity: Entity) {
        self.entitiy_storage.despawn(entity);
        for (type_id, storage) in self.component_storage.iter() {
            if storage.remove_untyped(entity).is_some() {
                self.events.publish(WorldEvent::ComponentRemoved(entity, *type_id))
            }
        }
    }

    // components

    pub fn get_component_set<T: Component + 'static>(&self) -> Option<Ref<ComponentSet<T>>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &ComponentCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow())
    }
    pub fn get_component_set_mut<T: Component + 'static>(&self) -> Option<RefMut<ComponentSet<T>>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &ComponentCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow_mut())
    }
    fn insert_component_storage<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let set = ComponentSet::<T>::new();
        let storage = ComponentCell { inner: RefCell::new(set) };
        self.component_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
    pub fn insert_component<T: Component + 'static>(
        &mut self,
        entity: Entity,
        component: T
    ) -> Result<(), EntityError> {
        let type_id = TypeId::of::<T>();
        if !self.component_storage.contains_key(&type_id) {
            self.insert_component_storage::<T>()
        }
        let res = self.get_component_set_mut()
            .ok_or(EntityError)?
            .insert(entity, component);
        if res.is_ok() { self.events.publish(WorldEvent::ComponentSpawned(entity, type_id)) }
        res
    }
    pub fn remove_component<T: Component + 'static>(&mut self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let res = self.get_component_set_mut()?
            .remove(entity);
        if res.is_some() { self.events.publish(WorldEvent::ComponentRemoved(entity, type_id)) }
        res
    }

    // resources

    pub fn get_resource<T: 'static>(&self) -> Option<Ref<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.resource_storage.get(&type_id)?;
        let cell: &ResourceCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow())
    }
    pub fn get_resource_mut<T: 'static>(&self) -> Option<RefMut<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &ResourceCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow_mut())
    }
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        let storage = ResourceCell { inner: RefCell::new(resource) };
        self.resource_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
}

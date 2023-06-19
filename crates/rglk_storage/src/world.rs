use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData
};

use super::{IdSize, Entity};
use super::sparse::{SparseSet, StorageCell};

// macro_rules! components {
//     ($world: expr, $entities:expr, $($t:ty),+) => {
//         (
//             $(
//                 $world.get_component_storage::<$t>().unwrap().get_many($entities)
//             ,)+
//         )
//     };
// }
macro_rules! components {
    ($world: expr, $entities:expr, $($t:ty),+) => {
        $entities.map(|e| ($(
                e,
                $world.get_component_storage::<$t>().unwrap().get(*e)
            ),+))
    }
}
macro_rules! common_entities {
    ($world:expr, $a:ty, $($t:ty),*) => {
        $world.get_component_storage::<$a>().unwrap().entities().iter()
            $(
                .filter(|e| $world.get_component_storage::<$t>().unwrap().entities().contains(e))
            )*
    }
}
macro_rules! query {
    ($world:expr, $($t:ty),+) => {{
        components!($world, common_entities!($world, $($t),+), $($t),+)
    }}
}

pub struct World {
    component_storage: HashMap<TypeId, Box<dyn Any>>
}
impl World {
    pub fn get_component_storage<T: 'static>(&self) -> Option<&SparseSet<T>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        storage.downcast_ref()
    }
    pub fn get_component_storage_mut<T: 'static>(&mut self) -> Option<&SparseSet<T>> {
        // mutabilit only for testing now
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        storage.downcast_ref()
    }
    fn insert_component_storage<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let set = SparseSet::<T>::new();
        let storage = StorageCell { inner: RefCell::new(set) };
        self.component_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
    pub fn insert_component<T: 'static>(&mut self, entity: Entity, component: T) {
        // let cell = 
    }
    pub fn query(&mut self) {
        let q1 = query!(self, u32, String);
        let q2 = query!(self, u32, String, f32); 
        let q3 = query!(self, u32, String, f32, u64); 
    }
}

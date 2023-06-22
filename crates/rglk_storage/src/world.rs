use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    fmt::Display,
    marker::PhantomData,
    ops::Deref,
    rc::Rc
};

use super::{IdSize, Entity};
use super::errors::EntityError;
use super::sparse::{SparseSet, StorageCell};

#[macro_export]
macro_rules! components {
    ($world: expr, $entities:expr, $($T:ty),+) => {{
        $entities.map(|e| ($(
            $world.get_component_storage::<$T>().unwrap().get(*e).unwrap()
        ),+))
    }}
}
// fn intersect_predcate<'a, T: Display + 'static>(world: &'a World) -> impl FnMut(&'a &'a Entity) -> bool {
//     let entities = world.get_component_storage::<T>().unwrap().entities();
//     |e: && Entity| {
//         entities.contains(e)
//     }
// }
// macro_rules! common_entities {
//     ($world:expr, $a:ty, $($t:ty),*) => {
//         // TODO reduce multiple get_component_storage calls
//         $world.get_component_storage::<$a>().unwrap().entities().iter()
//             $(
//                 // .filter(|e| $world.get_component_storage::<$t>().unwrap().entities().contains(e))
//                 .filter(intersect_predcate::<$t>($world))
//             )*
//     }
// }
// macro_rules! query {
//     ($world:expr, $($t:ty),+) => {{
//         components!($world, common_entities!($world, $($t),+), $($t),+)
//     }}
// }


use super::sparse::Storage;

pub struct World {
    component_storage: HashMap<TypeId, Box<dyn Storage>>
}
impl World {
    pub fn new() -> Self {
        World { component_storage: HashMap::new() }
    }
    // pub fn get_component_storage<T: Display + 'static>(&self) -> Option<Ref<'_, SparseSet<T>>> {
    //     let type_id = TypeId::of::<T>();
    //     let storage = self.component_storage.get(&type_id)?;
    //     let cell: &StorageCell<T> = storage.as_any().downcast_ref()?;
    //     Some(cell.inner.borrow())
    // }
    pub fn get_component_storage<T: Display + 'static>(&self) -> Option<impl Deref<Target=SparseSet<T>> + '_> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &StorageCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow())
    }
    pub fn get_component_storage_mut<T: Display + 'static>(&self) -> Option<RefMut<SparseSet<T>>> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id)?;
        let cell: &StorageCell<T> = storage.as_any().downcast_ref()?;
        Some(cell.inner.borrow_mut())
    }
    fn insert_component_storage<T: Display + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let set = SparseSet::<T>::new();
        let storage = StorageCell { inner: RefCell::new(set) };
        self.component_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
    pub fn insert_component<T: Display + 'static>(&mut self, entity: Entity, component: T) -> Result<(), EntityError> {
        let type_id = TypeId::of::<T>();
        if !self.component_storage.contains_key(&type_id) {
            self.insert_component_storage::<T>()
        }
        self.get_component_storage_mut::<T>()
            .ok_or(EntityError)?.insert(entity, component)
    }
    pub fn query(&mut self) {
        // let q1 = query!(self, u32, String);
        // let q2 = query!(self, u32, String, f32); 
        // let q3 = query!(self, u32, String, f32, u64); 
    }
    // pub fn get_desc<T>(&self) {
    //     for (k, v) in self.component_storage.iter() {
    //         let s = v.as_ref();
    //         let c = s.get_desc(Entity { id: 0, version: 0});
    //         c.desc();
    //     }
    // }
    // pub fn query_two<'a, M: Display + 'static, N: Display + 'static>(&'a self) -> impl Iterator<Item=(Entity, Ref<'a, &'a M>)> {
    //     let storage_m: Ref<SparseSet<M>> = self.get_component_storage::<M>().unwrap().inner.borrow();
    //     let storage_n: Ref<SparseSet<N>> = self.get_component_storage::<N>().unwrap().inner.borrow();
    //     // let entities = storage_m.inner.borrow().entities();
    //     storage_m.entities().iter()
    //         .map(|e| (*e, Ref::map(&storage_m, |s: &SparseSet<M>| &s.get(*e).unwrap())))
    //         // .map(|e| (e, storage_m.get(*e).unwrap()))
    //         // .map(|(e, c)| (e, c, storage_n.get(*e).unwrap()))
    // }
}

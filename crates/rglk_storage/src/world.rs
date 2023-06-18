use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    marker::PhantomData
};

use super::IdSize;
use super::sparse::SparseSet;

// macro_rules! query_single {
//     ($world:expr, $entity:expr, $($t:ty),+) => {
//         (
//             $(
//                 $world.get_component_storage::<$t>().get($entity).downcast_ref().unwrap()
//             ,)+
//         )
//     };
// }

// fn intersect(a: &[IdSize], b: &[IdSize]) -> Vec<IdSize> {
//     let h: HashSet<&IdSize> = HashSet::from_iter(a);
//     // TODO do not allocate
//     b.iter().filter(|e| h.contains(e)).map(|e| *e).collect()
// }

// macro_rules! components {
//     ($world: expr, $entities:expr, $($t:ty),+) => {
//         (
//             $(
//                 $world.get_component_storage::<$t>().get_many($entities)
//             ,)+
//         )
//     };
// }

// macro_rules! query {
//     ($world:expr, $a:ty, $b:ty) => {{
//         let entities = &intersect(
//                 $world.get_component_storage::<$a>().entities(),
//                 $world.get_component_storage::<$b>().entities()
//         )[..];
//         components!($world, entities, $a, $b)
//     }};
//     ($world:expr, $a:ty, $b:ty, $($t:ty),+) => {{
//         // let entities = $(
//         //     intersect(
//         //         $world.get_component_storage::<$t>().entities(),
//         //         &query!($world, $a, $b)[..]
//         //     );
//         // )+
//         let entities: Vec<u16> = vec![0, 1];
//         components!($world, &entities[..], $a, $b, $($t)+)
//     }};
// }

pub struct World {
    component_storage: HashMap<TypeId, Box<dyn Any>>
}
impl World {
    pub fn get_component_storage<T: 'static>(&self) -> &SparseSet<T> {
        let type_id = TypeId::of::<T>();
        let storage = self.component_storage.get(&type_id).unwrap();
        storage.downcast_ref().unwrap()
    }
    pub fn insert_component_storage<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let storage = SparseSet::<T>::new();
        self.component_storage.insert(
            type_id,
            Box::new(storage)
        );
    }
    pub fn query(&self) {
        // let e = super::Entity { id: 0, version: 0 };
        // let q0: (&u32, &String) = query_single!(self, e, u32, String);
        // let q1 = query!(self, u32, String);
        // let q2 = query!(self, u32, String, f32);
    }
}

// pub struct Query<'a> {
//     world: &'a mut World
// }
// impl<'a> Query<'a> {
//     pub fn new(world: &'a mut World) -> Self {
//         Query { world }
//     }
//     pub fn iter<T: Storage>(&mut self) -> QueryIter<T> {
//         // let _ = self.world.get_component_storage::<T>();
//         // self
//         QueryIter::<T> { world: self.world, phantom: PhantomData }
//     }
// }
// pub struct QueryIter<'a, T: Storage> {
//     world: &'a mut World,
//     phantom: PhantomData<&'a T>,
//     entities: &'a [IdSize],
//     // components: 
// }
// impl<'a, T: Storage> QueryIter<'a, T> {
//     pub fn chain<N: Storage>(&mut self, ) -> QueryIter<(T, N)> {
//         QueryIter::<(T, N)> { world: self.world, phantom: PhantomData }
//     }
// }


// pub struct Query {
//     types: [TypeId; 8],
//     // storages: Vec<Box<dyn Storage>>,
//     count: usize
// }
// impl Query {
//     pub fn new() -> Self {
//         Query {
//             types: [TypeId::of::<()>(); 8],
//             count: 0
//         }
//     }
//     pub fn add<T: 'static>(&mut self) {
//         self.types[self.count] = TypeId::of::<T>();
//         self.count += 1;
//     }
//     pub fn iter(&mut self, world: &World) {
//         let lengths = (0..self.count)
//             .map(|i| world.get_s(self.types[i]).len());
//     }
// }
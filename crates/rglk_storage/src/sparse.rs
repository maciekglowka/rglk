
use super::{Entity, IdSize};
use super::errors::EntityError;

const GUARD_ID: IdSize = IdSize::MAX;
// const GUARD_ENTITY: Entity = Entity { id: GUARD_ID, version: GUARD_ID };

pub struct SparseSet<T> {
    dense: Vec<Entity>,
    sparse: Vec<IdSize>,
    entries: Vec<T>
}
impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet { dense: Vec::new(), sparse:Vec::new (), entries: Vec::new() }
    }
    fn get_dense_index(&self, entity: Entity) -> Option<usize> {
        let index = *(self.sparse.get(entity.id as usize)?) as usize;
        // verify if the entity version is not mismatch
        match *self.dense.get(index)? == entity {
            false => None,
            true => Some(index)
        }
    }
    pub fn insert(&mut self, entity: Entity, entry: T) -> Result<(), EntityError> {
        // On conflict do nothing
        let index = entity.id as usize;
        if index >= self.sparse.len() {
            // add some extra buffer to minimize number of resizes?
            self.sparse.resize(index + 1, GUARD_ID);
        } else if self.sparse[index] != GUARD_ID {
            // already assigned
            return Err(EntityError);
        }
        self.sparse[index] = self.dense.len() as IdSize; 

        // those two vecs have to be kept in sync
        self.dense.push(entity);
        self.entries.push(entry);
        Ok(())
    }
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let removed_pos = self.get_dense_index(entity)?;
        // if there are no elements we have already returned above
        let last_pos = self.dense.len() - 1;
        let swapped_sparse_idx = self.dense[last_pos].id as usize;

        // swap the removed entry with the last one
        self.dense.swap(removed_pos, last_pos);
        self.entries.swap(removed_pos, last_pos);

        // remove the last element
        self.dense.pop();
        let removed = self.entries.pop();

        // fix the sparse vec to point to the swapped entry
        self.sparse[swapped_sparse_idx] = removed_pos as IdSize;
        // this goes last, in case the removed value was swapped with itself
        self.sparse[entity.id as usize] = GUARD_ID;
        removed
    }
    pub fn get(&self, entity: Entity) -> Option<&T> {
        Some(self.entries.get(
            self.get_dense_index(entity)?
        )?)
    }
    // pub fn get_many(&self, e: &[IdSize]) -> Vec<&T> {
    //     // does not check if element exists!
    //     // TODO handle entities properly
    //     e.iter()
    //         .map(|a| &self.entries[*a as usize])
    //         .collect()
    // }
}
// impl<T: 'static> Storage for SparseSet<T> {
//     fn len(&self) -> usize {
//         self.dense.len()
//     }
//     fn as_any(&self) -> &dyn std::any::Any { self }
//     fn get(&self, entity :crate::Entity) -> &dyn std::any::Any {
//         // TODO
//         &self.entries[entity.id as usize]
//     }
//     fn entities(&self) -> &[IdSize] {
//         &self.dense[..]
//     }
//     // fn get_many(&self, e: &[IdSize]) -> Vec<&dyn Any> {
//     //     // does not check if element exists!
//     //     // TODO handle entities properly
//     //     e.iter()
//     //         .map(|a| self.get(Entity { id: *a, version: 0 }))
//     //         .collect()
//     // }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_to_empty() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        let entry = "TEST_ENTRY";
        let res = set.insert(entity, entry);
        assert!(res.is_ok());
        assert!(set.dense[0] == entity);
        assert!(set.entries[0] == entry);
        assert!(set.sparse[0] == 0);
    }

    #[test]
    fn insert_unordered() {
        let count = 10;
        let mut set = SparseSet::<String>::new();
        for i in 0..count {
            let _ =set.insert(
                Entity { id: i * 4, version: 0 },
                format!("ENTRY_{}", i * 4)
            );
        }
        let entry: String = "TESTED".into();
        let entity = Entity { id: 7, version: 0 };
        let res = set.insert(entity, entry.clone());
        assert!(res.is_ok());
        let idx = set.sparse[7];
        assert!(idx as usize == set.dense.len() - 1);
        assert!(count as usize + 1 == set.dense.len());
        assert!(set.dense[idx as usize] == entity);
        assert!(set.entries[idx as usize] == entry);
    }

    #[test]
    fn remove_from_middle() {
        let count = 10;
        let mut set = SparseSet::<String>::new();
        for i in 0..count {
            let _ =set.insert(
                Entity { id: i * 4, version: 0 },
                format!("ENTRY_{}", i * 4)
            );
        }
        let removed_entity = Entity { id: 4, version: 0};
        let removed = set.remove(removed_entity);
        assert!(removed == Some("ENTRY_4".into()));
        assert!(count as usize - 1 == set.dense.len());
        assert!(count as usize - 1 == set.entries.len());
        for i in 0..count {
            let entity = Entity { id: i * 4, version: 0};
            if removed_entity == entity {
                assert!(set.get(entity).is_none());
            } else {
                assert!(set.get(entity).is_some());
            }
        }
    }
    #[test]
    fn remove_last() {
        let count = 10;
        let mut set = SparseSet::<String>::new();
        for i in 0..count {
            let _ =set.insert(
                Entity { id: i * 4, version: 0 },
                format!("ENTRY_{}", i * 4)
            );
        }
        let removed_entity = Entity { id: 4 * 9, version: 0};
        let removed = set.remove(removed_entity);
        assert!(removed == Some("ENTRY_36".into()));
        assert!(count as usize - 1 == set.dense.len());
        assert!(count as usize - 1 == set.entries.len());
        for i in 0..count {
            let entity = Entity { id: i * 4, version: 0};
            if entity == removed_entity {
                assert!(set.get(entity).is_none());
            } else {
                assert!(set.get(entity).is_some());
            }
        }
    }
    #[test]
    fn remove_only() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        let entry = "TEST_ENTRY";
        let _ = set.insert(entity, entry);
        let removed = set.remove(entity);
        assert!(removed == Some("TEST_ENTRY"));
        assert!(set.dense.len() == 0);
        assert!(set.entries.len() == 0);
    }
    #[test]
    fn remove_from_empty() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        assert!(None == set.remove(entity));
    }
    #[test]
    fn get() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        let entry = "TEST_ENTRY";
        let _ = set.insert(entity, entry);
        assert!(set.get(entity) == Some(&"TEST_ENTRY"));
    }
    #[test]
    fn get_not_existing() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        let wrong_entity = Entity { id: 3, version: 0 };
        let entry = "TEST_ENTRY";
        let _ = set.insert(entity, entry);
        assert!(set.get(wrong_entity).is_none());
    }
    #[test]
    fn get_version_mismatch() {
        let mut set = SparseSet::<&str>::new();
        let entity = Entity { id: 0, version: 0 };
        let wrong_entity = Entity { id: 0, version: 3 };
        let entry = "TEST_ENTRY";
        let _ = set.insert(entity, entry);
        assert!(set.get(wrong_entity).is_none());
    }
}

use super::IdSize;
use super::errors::EntityError;

const GUARD: IdSize = IdSize::MAX;

pub struct SparseSet<T> {
    dense: Vec<IdSize>,
    sparse: Vec<IdSize>,
    entries: Vec<T>
}
impl<T> SparseSet<T> {
    pub fn new() -> Self {
        SparseSet { dense: Vec::new(), sparse:Vec::new (), entries: Vec::new() }
    }
    pub fn insert(&mut self, id: IdSize, entry: T) -> Result<(), EntityError> {
        // On conflict do nothing
        let index = id as usize;
        if index >= self.sparse.len() {
            // add some extra buffer to minimize number of resizes?
            self.sparse.resize(index, GUARD);
        } else if self.sparse[index] != GUARD {
            // already assigned
            return Err(EntityError);
        }
        self.sparse[index] = self.dense.len() as IdSize; 
        self.dense.push(id);
        self.entries.push(entry);
        Ok(())
    }
    pub fn remove(&mut self, id: IdSize) {
        let index = id as usize;
        let last = self.dense.len() - 1;
        let removed = self.sparse[index] as usize;
        let swapped_index = self.dense[last] as usize;

        // swap the removed entry with the last one
        self.dense.swap(removed, last);
        self.entries.swap(removed, last);
        // remove the last element
        self.dense.pop();
        self.entries.pop();

        // fix the sparse vec to point to the current entry
        self.sparse[swapped_index] = removed as IdSize;
        // this goes last, in case the removed value was swapped with itself
        self.sparse[index] = GUARD;
    }
}

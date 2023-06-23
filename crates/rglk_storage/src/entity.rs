pub type IdSize = u16;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Entity {
    pub id: IdSize,
    pub version: IdSize
}

pub struct EntityFilter<T> {
    inner: T
}
impl EntityFilter<Box<std::slice::Iter<'_, Entity>>> {
    pub fn from(entities: &[Entity]) -> EntityFilter<Box<dyn Iterator<Item=Entity> + '_>> {
        EntityFilter { inner: Box::new(entities.iter().map(|e| *e)) }
    }
}
impl<'a, T: Iterator<Item=Entity> + 'a> EntityFilter<T> {
    pub fn combine(self, entities: &'a [Entity]) -> EntityFilter<Box<dyn Iterator<Item=Entity> + 'a>> {
        EntityFilter {
            inner: Box::new(self.inner.filter(|e| entities.contains(e)))
        }
    }
}
impl<'a, T: Iterator<Item=Entity> + 'a> Iterator for EntityFilter<T> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

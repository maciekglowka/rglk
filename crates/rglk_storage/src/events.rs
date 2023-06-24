use std::any::TypeId;
use super::Entity;

pub struct EventBus<T: Copy> {
    pub subscribers: Vec<fn(T)>,
}
impl<T: Copy> EventBus<T> {
    pub fn new() -> Self {
        EventBus { subscribers: Vec::new() }
    }
    pub fn publish(&self, e: T) {
        for s in self.subscribers.iter() {
            s(e);
        }
    }
    pub fn subscribe(&mut self, subscriber: fn(T)) {
        self.subscribers.push(subscriber);
    }
    pub fn unsubscribe(&mut self, subscriber: fn(T)) {
        self.subscribers.retain(|&a| a != subscriber);
    }
}


#[derive(Clone, Copy)]
pub enum WorldEvent {
    ComponentSpawned(Entity, TypeId),
    ComponentRemoved(Entity, TypeId)
}
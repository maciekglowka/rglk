use std::{
    collections::HashMap,
    rc::{Rc, Weak},
    sync::Mutex
};

// at the moment not thread safe

pub struct SubscriberHandle<T: Copy> {
    queue: Weak<Mutex<Vec<T>>>,
    pub id: usize
}
impl<'a, T: Copy> SubscriberHandle<T> {
    pub fn read(&self) -> Option<Vec<T>> {
        let strong = self.queue.upgrade()?;
        let mut queue = strong.lock().ok()?;
        Some(queue.drain(..).collect())
    }
}

pub struct EventBus<T: Copy> {
    subscribers: HashMap<usize, Rc<Mutex<Vec<T>>>>,
    next: usize
}
impl<'a, T: Copy> EventBus<T> {
    pub fn new() -> Self {
        EventBus { subscribers: HashMap::new(), next: 0 }
    }
    pub fn publish(&mut self, e: T) {
        for (_, s) in self.subscribers.iter_mut() {
            if let Ok(mut v) = s.lock() {
                v.push(e);
            }
        }
    }
    pub fn subscribe(&mut self) -> SubscriberHandle<T> {
        let queue = Rc::new(Mutex::new(Vec::new()));
        let id = self.next;
        let handle = SubscriberHandle{
            id,
            queue: Rc::downgrade(&queue)
        };
        self.subscribers.insert(id, queue);
        self.next += 1;
        handle
    }
    pub fn unsubscribe(&mut self, handle: SubscriberHandle<T>) {
        self.subscribers.retain(|k, _| *k != handle.id);
    }
}

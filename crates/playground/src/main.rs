use std::cell::Ref;
use std::time::Instant;
use rglk_storage::{Entity, EntityFilter, SparseSet, World};

fn main() {
    let mut world = World::new();

    let start = Instant::now();
    for i in 0..10000 {
        let entity = Entity { id: i, version: 0};
        world.insert_component::<u32>(entity, 0).unwrap();
        world.insert_component::<String>(entity, "-5".into()).unwrap();
    }
    println!("{:?}", start.elapsed());

    let start = Instant::now();

    system(&world.get_component_storage::<u32>().unwrap(), &world.get_component_storage::<String>().unwrap());

    println!("{:?}", start.elapsed());
}

fn system(a: &SparseSet<u32>, b: &SparseSet<String>) {
    for e in EntityFilter::from(a.entities()).combine(b.entities()) {
        a.get(*e);
        b.get(*e);
    }
}

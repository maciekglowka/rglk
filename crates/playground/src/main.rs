use std::time::Instant;
use rglk_storage::{Entity, EntityFilter, SparseSet, World};

fn main() {
    let mut world = World::new();

    let start = Instant::now();
    for i in 0..10000 {
        let entity = Entity { id: i, version: 0};
        world.insert_component::<u32>(entity, i as u32).unwrap();
        world.insert_component::<String>(entity, "-5".into()).unwrap();
    }
    println!("{:?}", start.elapsed());

    let start = Instant::now();

    // system(&world.get_component_storage::<u32>().unwrap(), &world.get_component_storage::<String>().unwrap());
    // println!("{:?}", start.elapsed());    

    system_mut(&world.get_component_storage::<u32>().unwrap(), &mut world.get_component_storage_mut::<String>().unwrap());
    println!("{:?}", start.elapsed());

}

fn system(a: &SparseSet<u32>, b: &SparseSet<String>) {
    let mut s = 0;
    for e in EntityFilter::from(a.entities()).combine(b.entities()) {
        s += a.get(e).unwrap();
        b.get(e);
    }
}

fn system_mut(a: &SparseSet<u32>, b: &mut SparseSet<String>) {
    let mut s = 0;
    for e in EntityFilter::from(a.entities()).combine(b.entities()).collect::<Vec<_>>().iter() {
        s += a.get(*e).unwrap();
        b.get_mut(*e);
    }
}

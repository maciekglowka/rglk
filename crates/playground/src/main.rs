use std::time::Instant;
use rglk_storage::{Entity, EntityFilter, Component, ComponentSet, World};

fn main() {
    let mut world: World = World::new();

    let start = Instant::now();
    for i in 0..1000 {
        let entity = Entity { id: i, version: 0};
        world.insert_component::<Health>(entity, Health(i as u32)).unwrap();
        // world.insert_component::<String>(entity, "-5".into()).unwrap();
    }
    for i in 0..500 {
        let entity = Entity { id: i, version: 0};
        // world.insert_component::<u32>(entity, i as u32).unwrap();
        world.insert_component::<Name>(entity, Name("Name".into())).unwrap();
    }
    println!("{:?}", start.elapsed());

    let start = Instant::now();

    system(&world.get_component_set::<Health>().unwrap(), &world.get_component_set::<Name>().unwrap());
    // println!("{:?}", start.elapsed());    

    // system_mut(&world.get_component_storage::<u32>().unwrap(), &mut world.get_component_storage_mut::<String>().unwrap());
    println!("{:?}", start.elapsed());

}

fn system(a: &ComponentSet<Health>, b: &ComponentSet<Name>) {
    let mut s = 0;
    for e in EntityFilter::from(a.entities()).combine(b.entities()) {
        s += a.get(e).unwrap().0;
        b.get(e);
    }
}

// fn system_mut(a: &ComponentSet<u32>, b: &mut ComponentSet<String>) {
//     let mut s = 0;
//     for e in EntityFilter::from(a.entities()).combine(b.entities()).collect::<Vec<_>>().iter() {
//         // for e in EntityFilter::from(a.entities()) {
//         s += a.get(*e).unwrap();
//         b.get_mut(*e).unwrap();
//     }
// }
struct Health(u32);
impl Component for Health {
    fn as_str(&self) -> String {
        format!("HP: {}", self.0)
    }
}

struct Name(String);
impl Component for Name {}
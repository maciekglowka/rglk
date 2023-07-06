// use std::collections::VecDeque;

use::rglk_storage::{Entity, World};

use super::actions::{Action, ActorQueue, Damage, Pause, PendingActions};
use super::components::{Actor, Card, Health, Melee, Player, PlayerCharacter, Position, Projectile};
use super::wind::Wind;

pub fn game_step(world: &mut World) {
    hit_projectiles(world);
    hit_melee(world);
    let pending_result = process_pending_actions(world);
    kill_units(world);
    // do not process the actor queue if the pending actions were executed
    if pending_result.is_some() { return };
    let Some(actor) = get_current_actor(world) else {
        turn_end(world);
        return
    };
    let action = process_actor(actor, world);
    if let Some(_action) = action {
        // if we reached this point it should be safe to unwrap
        // on the actor queue
        world.get_resource_mut::<ActorQueue>().unwrap().0.pop_front();
    }
}

fn process_pending_actions(world: &mut World) -> Option<Vec<Box<dyn Action>>> {
    // returns executed actions from the queue
    let pending = world.get_resource_mut::<PendingActions>()?.0.drain(..).collect::<Vec<_>>();
    let mut resulting = Vec::new();
    let mut output = Vec::new();
    for action in pending {
        let res = action.execute(world);
        if let Some(res) = res {
            resulting.extend(res);
        }
        output.push(action);
    }
    match resulting.len() {
        0 => None,
        _ => Some(resulting)
    }
}

fn get_current_actor(world: &mut World) -> Option<Entity> {
    let queue = world.get_resource::<ActorQueue>()?;
    queue.0.get(0).map(|&e| e)
}

fn process_actor(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    // returns a succesfully performed action or None
    let action = get_action(entity, world)?;
    action.execute(world);
    Some(action)
}

fn get_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let Some(mut actor) = world.get_component_mut::<Actor>(entity) else {
        // remove actor from the queue as it might have been killed or smth
        world.get_resource_mut::<ActorQueue>()?.0.retain(|a| *a != entity);
        return None;
    };
    if let Some(action) = actor.action.take() { return Some(action) };
    if world.get_component::<PlayerCharacter>(entity).is_some() { return None };

    // choose npcs actions
    let mut possible_actions = actor.cards.iter()
        .filter_map(|e| world.get_component::<Card>(*e))
        .map(|c| c.0.get_possible_actions(entity, world).drain().collect::<Vec<_>>())
        .flatten()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    possible_actions.sort_by(|a, b| a.score(world).cmp(&b.score(world)));
    match possible_actions.pop() {
        Some(a) => Some(a),
        _ => Some(Box::new(Pause))
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().iter().map(|a| a.entity).collect::<Vec<_>>();
    actors.sort_by(|a, b| a.id.cmp(&b.id));
    queue.0 = actors.into();
}

fn hit_melee(world: &mut World) {
    let melee_query = world.query::<Melee>().with::<Position>();
    let health_query = world.query::<Health>().with::<Position>();

    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };

    for item in melee_query.iter() {
        let position = item.get::<Position>().unwrap();
        let targets = health_query.iter()
            .filter(|a| a.get::<Position>().unwrap().0.manhattan(position.0) == 1)
            .collect::<Vec<_>>();
        for target in targets {
            if !are_hostile(item.entity, target.entity, world) { continue; }
            let damage = item.get::<Melee>().unwrap().0;
            pending.0.push(
                Box::new(Damage { entity: target.entity, value: damage })
            );
        }
    }
}

fn hit_projectiles(world: &mut World) {
    // this should be called before actions are exectued
    // to clear projectiles spawned at the previous tick
    let query = world.query::<Projectile>();
    let health_query = world.query::<Health>().with::<Position>();

    if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
        for item in query.iter() {
            let projectile = item.get::<Projectile>().unwrap();
            let target = health_query.iter()
                .filter(|a| a.get::<Position>().unwrap().0 == projectile.target)
                .next();
            if let Some(target) = target {
                pending.0.push(
                    Box::new(Damage { entity: target.entity, value: projectile.damage })
                );
            }
        }
    };

    let entities = query.iter()
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        world.despawn_entity(entity);
        
    }
}

fn kill_units(world: &mut World) {
    let query = world.query::<Health>();
    let entities = query.iter()
        .filter(|a| a.get::<Health>().unwrap().0 == 0)
        .map(|a| a.entity)
        .collect::<Vec<_>>();
    for entity in entities {
        world.despawn_entity(entity);
    }
}

fn turn_end(world: &mut World) {
    if let Some(mut wind) = world.get_resource_mut::<Wind>() {
        wind.pop_wind();
    }
    collect_actor_queue(world);
}

fn are_hostile(source: Entity, target: Entity, world: &World) -> bool {
    if world.get_component::<Player>(source).is_some() {
        return world.get_component::<Player>(target).is_none()
    } else {
        return world.get_component::<Player>(target).is_some()
    }
}
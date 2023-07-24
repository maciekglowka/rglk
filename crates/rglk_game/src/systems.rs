// use std::collections::VecDeque;

use::rogalik::storage::{Entity, World};

use super::GameManager;
use super::actions::{Action, ActorQueue, Damage, MeleeAttack, Pause, PendingActions, SelectedAction};
use super::components::{Actor, Card, Cooldown, Health, Melee, Player, PlayerCharacter, Position, Projectile};
use super::wind::Wind;

pub fn game_step(world: &mut World) {
    hit_projectiles(world);
    let pending_result = process_pending_actions(world);
    kill_units(world);
    // do not process the actor queue if the pending actions were executed
    if let Some(result) = pending_result { 
        // safe to unwrap
        world.get_resource_mut::<PendingActions>().unwrap().0 = result;
        return 
    };
    let Some(actor) = get_current_actor(world) else {
        turn_end(world);
        return
    };
    if process_actor(actor, world) {
        // if we reached this point it should be safe to unwrap
        // on the actor queue
        world.get_resource_mut::<ActorQueue>().unwrap().0.pop_front();
        try_melee(actor, world);
    }
}

fn process_pending_actions(world: &mut World) -> Option<Vec<Box<dyn Action>>> {
    // returns executed actions from the queue
    let pending = world.get_resource_mut::<PendingActions>()?.0.drain(..).collect::<Vec<_>>();
    let mut resulting = Vec::new();
    let mut output = Vec::new();
    for action in pending {
        let res = execute_action(&*action, world);
        if let Ok(res) = res {
            if let Some(next) = res {
                resulting.extend(next);
            }
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

fn process_actor(entity: Entity, world: &mut World) -> bool {
    // return true is succesful
    let Some(selected) = get_action(entity, world) else { return false };

    if let Ok(res) = execute_action(&*selected.action, world) {
        if let Some(next) = res {
            if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
                pending.0.extend(next);
            }
        }
        if let Some(card) = selected.card {
            if let Some(mut cooldown) = world.get_component_mut::<Cooldown>(card) {
                cooldown.current = cooldown.base;
            }
        }
    }
    true
}

fn get_action(entity: Entity, world: &mut World) -> Option<SelectedAction> {
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
        Some(a) => Some(SelectedAction { action: a, card: None }),
        _ => Some(SelectedAction { action: Box::new(Pause), card: None })
    }
}

fn collect_actor_queue(world: &mut World) {
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { return };
    let mut actors = world.query::<Actor>().iter().map(|a| a.entity).collect::<Vec<_>>();
    actors.sort_by(|a, b| a.id.cmp(&b.id));
    queue.0 = actors.into();
}

fn try_melee(entity: Entity, world: &mut World) {
    let Some(melee) = world.get_component::<Melee>(entity) else { return };
    let Some(position) = world.get_component::<Position>(entity) else { return };
    let Some(mut pending) = world.get_resource_mut::<PendingActions>() else { return };

    let targets = world.query::<Health>().with::<Position>().iter()
        .filter_map(|a| match a.get::<Position>() {
            Some(p) if p.0.manhattan(position.0) == 1 => Some((a.entity, p.0)),
            _ => None
        })

        .collect::<Vec<_>>();

    for target in targets {
        if !are_hostile(entity, target.0, world) { continue; }
        pending.0.push(
            Box::new(MeleeAttack { entity, target: target.1, damage: melee.0 })
        );
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
    reduce_cooldown(world);
    collect_actor_queue(world);
}

fn reduce_cooldown(world: &mut World) {
    for item in world.query::<Cooldown>().iter() {
        let mut cooldown = item.get_mut::<Cooldown>().unwrap();
        cooldown.current = cooldown.current.saturating_sub(1);
    }
}

fn are_hostile(source: Entity, target: Entity, world: &World) -> bool {
    if world.get_component::<Player>(source).is_some() {
        return world.get_component::<Player>(target).is_none()
    } else {
        return world.get_component::<Player>(target).is_some()
    }
}

fn execute_action(action: &dyn Action, world: &mut World) -> Result<Option<Vec<Box<dyn Action>>>, ()> {
    if let Some(mut state) = world.get_resource_mut::<GameManager>() {
        state.action_events.publish(super::ActionEvent(action.as_data()))
    }
    action.execute(world)
}
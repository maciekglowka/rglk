// use std::collections::VecDeque;

use::rglk_storage::{Entity, World};

use super::actions::{Action, ActorQueue, Pause};
use super::components::{Actor, Card, Player, Position};
use super::wind::Wind;

pub fn game_step(world: &mut World) {
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
    let mut actor = world.get_component_mut::<Actor>(entity)?;
    if let Some(action) = actor.action.take() { return Some(action) };
    if world.get_component::<Player>(entity).is_some() { return None };

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

fn turn_end(world: &mut World) {
    if let Some(mut wind) = world.get_resource_mut::<Wind>() {
        wind.pop_wind();
    }
    collect_actor_queue(world);
}
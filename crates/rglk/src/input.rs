use macroquad::prelude::*;

use rglk_graphics::globals::TILE_SIZE;
use rglk_math::vectors::Vector2I;


#[derive(Clone, Copy)]
pub enum InputAction {
    Direction(rglk_math::vectors::Vector2I),
    ChangeCard
}

pub fn get_mouse_tile(
    camera: &Camera2D,
) -> Vector2I {
    let mouse = mouse_position();
    let v = camera.screen_to_world(Vec2::new(mouse.0, mouse.1));
    Vector2I::new (
        (v.x / TILE_SIZE).floor() as i32,
        (v.y / TILE_SIZE).floor() as i32,
    )
}

pub fn get_input_action(camera: &Camera2D) -> Option<InputAction> {
    if is_key_down(KeyCode::Space) {
        return Some(InputAction::ChangeCard)
    }
    if is_mouse_button_pressed(MouseButton::Left) {
        return Some(InputAction::Direction(
            get_mouse_tile(camera)
        ))
    }
    None
}

pub fn handle_input(
    input: Option<InputAction>,
    world: &rglk_storage::World
) {
    if let Some(input) = input {
        let query = world.query::<rglk_game::components::PlayerCharacter>();
        let Some(item) = query.iter().next() else { return };
        let entity = item.entity;
        match input {
            InputAction::Direction(dir) => {
                let Some(mut actor) = item.get_mut::<rglk_game::components::Actor>() else { return };
                let player = item.get::<rglk_game::components::PlayerCharacter>().unwrap();
                let Some(card) = world.get_component::<rglk_game::components::Card>(actor.cards[player.active_card]) else { return };
                if let Some(action) = card.0.get_possible_actions(entity, world).remove(&dir) {
                    actor.action = Some(action);
                }
            },
            InputAction::ChangeCard => {
                let Some(actor) = item.get::<rglk_game::components::Actor>() else { return };
                let mut player = item.get_mut::<rglk_game::components::PlayerCharacter>().unwrap();
                player.active_card = (player.active_card + 1) % actor.cards.len();
            }
        }
    }
}
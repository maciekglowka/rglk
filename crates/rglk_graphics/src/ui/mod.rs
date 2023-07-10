use rglk_game::{
    components::{Actor, PlayerCharacter},
    Wind
};
use rglk_math::vectors::{Vector2I, Vector2F};
use rglk_storage::{ComponentSet, Entity, World, WorldEvent};

use super::{GraphicsState, GraphicsBackend, SpriteColor};

mod buttons;
mod cards;
// mod commands;

#[derive(Default)]
pub struct UiState {
    pub mouse_position: Vector2F,
    pub mouse_button_left: ButtonState
}

#[derive(Default)]
pub enum ButtonState {
    #[default]
    Up,
    Down,
    Pressed,
    Released
}

pub fn ui_update(
    world: &World,
    state: &mut GraphicsState,
    ui_state: UiState,
    backend: &dyn GraphicsBackend
) -> bool {
    let mut interaction = false;
    draw_wind_queue(world, backend);
    cards::handle_cards(world, backend, &ui_state);
    interaction
}


fn draw_wind_queue(world: &World, backend: &dyn GraphicsBackend) {
    let Some(wind) = world.get_resource::<Wind>() else { return };
    for (i, dir) in wind.queue.iter().enumerate() {
        let index = match *dir {
            Vector2I::DOWN => 31,
            Vector2I::UP => 30,
            Vector2I::LEFT => 17,
            Vector2I::RIGHT => 16,
            _ => continue
        };
        backend.draw_ui_sprite(
            "ascii",
            index,
            Vector2F::new(32. * i as f32, 0.),
            Vector2F::new(32., 32.),
            SpriteColor(255, 255, 255, 255)
        );
    }
}


// fn draw_cards(world: &World, backend: &dyn GraphicsBackend) {
//     let Some(cards) = world.query::<PlayerCharacter>().with::<Actor>()
//             .iter()
//             .map(|i| i.get::<Actor>().unwrap().cards.clone())
//             .next()
//             else { return };
//     let active = world.query::<PlayerCharacter>().iter()
//         .next()
//         .unwrap()
//         .get::<PlayerCharacter>()
//         .unwrap()
//         .active_card;

//     let viewport_size = backend.viewport_size();

//     for (i, card) in cards.iter().enumerate() {
//         let desc = world.get_entity_components(*card)
//             .iter()
//             .map(|c| c.as_str())
//             .collect::<Vec<_>>();
//         let desc = desc.join(", ");
//         let color = if i == active {
//             SpriteColor(255, 255, 255, 255)
//         } else {
//             SpriteColor(128, 128, 128, 255)
//         };
//         backend.draw_ui_text(
//             "default",
//             &desc,
//             Vector2F::new(
//                 32.,
//                 viewport_size.y - 32. * (i as f32 + 1.)
//             ),
//             32,
//             color
//         );
//     }
// }
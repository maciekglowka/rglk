use rglk_game::components::{Actor, PlayerCharacter};
use rglk_storage::World;

use super::{UiState, GraphicsBackend, SpriteColor};
use super::buttons::Button;

pub fn handle_cards(world: &World, backend: &dyn GraphicsBackend, state: &UiState) {
    let query = world.query::<PlayerCharacter>().with::<Actor>();
    let Some(item) = query.iter().next() else { return };
    let cards = &item.get::<Actor>().unwrap().cards;
    let active = item.get::<PlayerCharacter>().unwrap().active_card;

    let viewport_size = backend.viewport_size();

    for (i, card) in cards.iter().enumerate() {
        let desc = world.get_entity_components(*card)
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>();
        let desc = desc.join(", ");
        let color = if i == active {
            SpriteColor(255, 255, 255, 255)
        } else {
            SpriteColor(128, 128, 128, 255)
        };
        if Button::new(
                32.,
                viewport_size.y - 48. * (i as f32 + 1.),
                150.,
                32.
            )
            .with_text(&desc, SpriteColor(0, 0, 0, 255), 32)
            .with_color(color)
            .draw(backend)
            .clicked(state) {
                click_card(i, &mut item.get_mut::<PlayerCharacter>().unwrap())
            }
    }
}

fn click_card(index: usize, player: &mut PlayerCharacter) {
    player.active_card = index;
}
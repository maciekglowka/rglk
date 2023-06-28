use macroquad::prelude::*;

use rglk_game;
use rglk_graphics;
use rglk_sprites;
use rglk_storage;

#[macroquad::main("RGLK")]
async fn main() {
    let mut graphics_assets = rglk_sprites::Assets::new();
    rglk_graphics::assets::load_assets(&mut graphics_assets).await;

    let mut world = rglk_storage::World::new();
    set_camera(&Camera2D {
        zoom: Vec2::new(2. / screen_width(), -2. / screen_height()),
        ..Default::default()
    });
    let mut graphics_state = rglk_graphics::GraphicsState::new(
        &mut world,
        graphics_assets
    );
    rglk_game::init(&mut world);

    loop {
        handle_input(
            &world.get_component_set::<rglk_game::components::Player>().unwrap(),
            &mut world.get_component_set_mut::<rglk_game::components::Actor>().unwrap(),
        );

        rglk_game::game_step(&mut world);
        clear_background(BLACK);
        rglk_graphics::graphics_update(&world, &mut graphics_state);
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn handle_input(
    players: &rglk_storage::ComponentSet<rglk_game::components::Player>,
    actors: &mut rglk_storage::ComponentSet<rglk_game::components::Actor>
) {
    // let Some(entity) = rglk_storage::EntityFilter::from(players.entities())
    //     .combine(actors.entities())
    //     .next()
    //     else { return };
    // let Some(actor) = actors.get_mut(entity) else { return };
    // let mut action = None;
    // if is_key_down(KeyCode::A) {
    //     action = Some(rglk_game::actions::Walk{
    //         entity, direction: rglk_math::vectors::Vector2I::new(-1, 0)
    //     });
    // }
    // if is_key_down(KeyCode::D) {
    //     action = Some(rglk_game::actions::Walk{
    //         entity, direction: rglk_math::vectors::Vector2I::new(1, 0)
    //     });
    // }
    // if is_key_down(KeyCode::W) {
    //     action = Some(rglk_game::actions::Walk{
    //         entity, direction: rglk_math::vectors::Vector2I::new(0, -1)
    //     });
    // }
    // if is_key_down(KeyCode::S) {
    //     action = Some(rglk_game::actions::Walk{
    //         entity, direction: rglk_math::vectors::Vector2I::new(0, 1)
    //     });
    // }
    // if let Some(action) = action {
    //     actor.next = Some(Box::new(action));
    // }
}

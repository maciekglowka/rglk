use macroquad::prelude::*;
use std::{
    collections:: HashMap,
    time::{Duration, Instant}
};

use rglk_game;
use rglk_graphics;
use rglk_sprites;
use rglk_storage;

fn window_conf() -> Conf {
    Conf { 
        window_title: "RGLK".into(),
        window_width: 600,
        window_height: 800,
        ..Default::default()
    }
}

#[derive(Clone, Copy)]
enum InputAction {
    Direction(rglk_math::vectors::Vector2I)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut graphics_assets = rglk_sprites::Assets::new();
    rglk_graphics::assets::load_assets(&mut graphics_assets).await;

    let mut world = rglk_storage::World::new();
    let main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), -2. / screen_height()),
        target: 0.5 * rglk_graphics::globals::TILE_SIZE * Vec2::splat(8.),
        ..Default::default()
    };
    let mut graphics_state = rglk_graphics::GraphicsState::new(
        &mut world,
        graphics_assets
    );
    rglk_game::init(&mut world);

    let mut last_input = Instant::now();
    let mut last_action = None;

    loop {
        if let Some(action) = get_input_action() {
            last_action = Some(action)
        };

        rglk_game::game_step(&mut world);
        clear_background(BLACK);
        set_camera(&main_camera);
        let graphics_ready = rglk_graphics::graphics_update(&world, &mut graphics_state);
        set_default_camera();
        rglk_graphics::ui_update(&world, &mut graphics_state);
        next_frame().await;

        if graphics_ready && last_input.elapsed() > Duration::from_millis(200) {
            handle_input(last_action, &world);
            last_input = Instant::now();
            last_action = None;
        }

        // temp to save some cpu cycles
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn handle_input(
    input: Option<InputAction>,
    world: &rglk_storage::World
) {
    if let Some(input) = input {
        if let InputAction::Direction(dir) = input {
            let query = world.query::<rglk_game::components::Player>();
            let Some(entity) = query.iter().next() else { return };
            let entity = entity.entity;
            let Some(mut actor) = world.get_component_mut::<rglk_game::components::Actor>(entity) else { return };
            let Some(card) = world.get_component::<rglk_game::components::Card>(actor.cards[0]) else { return };
            if let Some(action) = card.0.get_possible_actions(entity, world).remove(&dir) {
                actor.action = Some(action);
            }
        }
    }
}

fn get_input_action() -> Option<InputAction> {
    let mut dir = None;
    if is_key_down(KeyCode::A) {
        dir = Some(rglk_math::vectors::Vector2I::new(-1, 0));
    }
    if is_key_down(KeyCode::D) {
        dir = Some(rglk_math::vectors::Vector2I::new(1, 0));
    }
    if is_key_down(KeyCode::W) {
        dir = Some(rglk_math::vectors::Vector2I::new(0, -1));
    }
    if is_key_down(KeyCode::S) {
        dir = Some(rglk_math::vectors::Vector2I::new(0, 1));

    }
    match dir {
        Some(d) => Some(InputAction::Direction(d)),
        _ => None
    }
}
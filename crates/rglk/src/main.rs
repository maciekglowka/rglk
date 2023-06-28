use macroquad::prelude::*;
use std::time::{Duration, Instant};

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
    set_camera(&Camera2D {
        zoom: Vec2::new(2. / screen_width(), -2. / screen_height()),
        target: 0.5 * rglk_graphics::globals::TILE_SIZE * Vec2::splat(rglk_game::globals::CHUNK_SIZE as f32),
        ..Default::default()
    });
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
        if last_input.elapsed() > Duration::from_millis(100) {
            handle_input(last_action, &world);
            last_input = Instant::now();
            last_action = None;
        }

        rglk_game::game_step(&mut world);
        clear_background(BLACK);
        rglk_graphics::graphics_update(&world, &mut graphics_state);
        next_frame().await;

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
            let query = world.query::<rglk_game::components::Player>()
                .with::<rglk_game::components::Actor>();
            if let Some(item) = query.iter().next() {
                let action = rglk_game::actions::Walk{
                    entity: item.entity, direction: dir
                };
                item.get_mut::<rglk_game::components::Actor>().unwrap().next = Some(Box::new(action));
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
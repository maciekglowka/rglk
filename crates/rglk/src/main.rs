use macroquad::prelude::*;
use std::{
    collections:: HashMap,
    time::{Duration, Instant}
};

use rglk_game;
use rglk_graphics;
use macroquad_sprites;
use rogalik::storage;

mod input;

fn window_conf() -> Conf {
    Conf { 
        window_title: "RGLK".into(),
        window_width: 600,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut backend = macroquad_sprites::MacroquadBackend::new();

    backend.load_atlas(
            "ascii",
            "assets/sprites/ascii.png",
            16,
            16,
            None
        ).await
        .expect("Could not load sprites!");

    backend.load_font("default",  "assets/ui/04B_03.ttf").await
        .expect("Could not find fonts!");

    let mut world = rogalik::storage::World::new();
    let mut main_camera = Camera2D {
        zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
        target: 0.5 * rglk_graphics::globals::TILE_SIZE * Vec2::splat(8.),
        ..Default::default()
    };
    let mut manager = rglk_game::GameManager::new();
    let mut graphics_state = rglk_graphics::GraphicsState::new(
        &mut world,
        &mut manager
    );
    rglk_game::init(&mut world, manager);

    // let mut last_input = Instant::now();
    let mut graphics_ready = true;
    // let mut ui_interaction = false;

    loop {
        let frame_start = Instant::now();

        if graphics_ready {
            // let start = Instant::now();
            rglk_game::game_step(&mut world);
            // println!("{:?}", start.elapsed()); 
        }
        clear_background(BLACK);
        // update_camera(&mut main_camera, &world);
        set_camera(&main_camera);
        backend.set_bounds(&main_camera);
        // let start = Instant::now();
        graphics_ready = rglk_graphics::graphics_update(&world, &mut graphics_state, &backend);
        // println!("{:?}", start.elapsed()); 
        set_default_camera();
        rglk_graphics::ui::ui_update(
            &mut world,
            input::get_input_state(&main_camera),
            &backend,
        );
        next_frame().await;

        // temp to save some cpu cycles
        std::thread::sleep(std::time::Duration::from_millis(16).saturating_sub(frame_start.elapsed()));
    }
}

// fn update_camera(
//     camera: &mut Camera2D,
//     world: &rglk_storage::World
// ) {
//     let pos = world.query::<rglk_game::components::PlayerCharacter>()
//         .iter()
//         .next()
//         .unwrap()
//         .get::<rglk_game::components::Position>()
//         .unwrap()
//         .0;
//     let player_v = (pos.as_f32() + rglk_math::vectors::Vector2F::new(0.5, 0.5)) * rglk_graphics::globals::TILE_SIZE;
//     let v = rglk_graphics::move_towards(
//         rglk_math::vectors::Vector2F::new(camera.target.x, camera.target.y),
//         player_v,
//         2.
//     );
//     camera.target = Vec2::new(v.x, v.y);
// }

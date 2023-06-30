pub mod assets;
mod renderers;
pub mod globals;
mod ui;

use rglk_game::components::{Position};
use rglk_events::SubscriberHandle;
use rglk_storage::{World, WorldEvent};

pub struct GraphicsState {
    pub assets: rglk_sprites::Assets,
    // TODO use sparse storage ?
    pub sprites: Vec<renderers::SpriteRenderer>,
    pub ev_world: SubscriberHandle<WorldEvent>
}
impl GraphicsState {
    pub fn new(world: &mut World, assets: rglk_sprites::Assets) -> Self {
        GraphicsState { 
            assets: assets,
            sprites: Vec::new(),
            ev_world: world.events.subscribe()
        }
    }
    pub fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }
}

pub fn graphics_update(
    world: &World,
    state: &mut GraphicsState
) {
    renderers::spawn_sprites(world, state);
    renderers::update_sprites(
        &world.get_component_set::<Position>().unwrap(),
        state
    );
    renderers::draw_sprites(state);
}

pub fn ui_update(
    world: &World,
    state: &mut GraphicsState   
) {
    ui::draw_wind_queue(world, state);
}

pub mod assets;
mod renderers;
mod globals;

use rglk_game::components::{Piece, Position, Tile};
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
    let positions = world.get_component_set::<Position>().unwrap();
    let pieces = world.get_component_set::<Piece>().unwrap();
    let tiles = world.get_component_set::<Tile>().unwrap();
    renderers::spawn_sprites(&positions, &pieces, &tiles, state);
    renderers::draw_sprites(state);
}
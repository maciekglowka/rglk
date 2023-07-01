pub mod assets;
pub mod globals;
mod graphics;
mod ui;

use rglk_game::components::{Position};
use rglk_events::SubscriberHandle;
use rglk_storage::{World, WorldEvent};

pub use graphics::graphics_update;
pub use ui::ui_update;

pub struct GraphicsState {
    pub assets: rglk_sprites::Assets,
    // TODO use sparse storage ?
    pub sprites: Vec<graphics::renderers::SpriteRenderer>,
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

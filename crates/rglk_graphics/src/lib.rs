pub mod globals;
mod graphics;
pub mod ui;

use rglk_game::{ActionEvent, GameManager};
use rglk_events::SubscriberHandle;
use rglk_math::vectors::Vector2F;
use rglk_storage::{World, WorldEvent};

pub use graphics::{
    graphics_update,
    utils::move_towards
};

pub struct GraphicsState {
    sprites: Vec<graphics::renderers::SpriteRenderer>,
    ev_world: SubscriberHandle<WorldEvent>,
    ev_actions: SubscriberHandle<ActionEvent>
}
impl GraphicsState {
    pub fn new(world: &mut World, manager: &mut GameManager) -> Self {
        GraphicsState { 
            sprites: Vec::new(),
            ev_world: world.events.subscribe(),
            ev_actions: manager.action_events.subscribe(),
        }
    }
    pub fn sort_sprites(&mut self) {
        self.sprites.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    }
}

pub trait GraphicsBackend {
    fn draw_world_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    );
    fn draw_ui_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    );
    fn draw_ui_text(
        &self,
        font_name: &str,
        text: &str,
        position: Vector2F,
        font_size: u32,
        color: SpriteColor
    );
    fn viewport_size(&self) -> Vector2F;
}

#[derive(Clone, Copy)]
pub struct SpriteColor(pub u8, pub u8, pub u8, pub u8);
use std::collections::HashMap;
use macroquad::prelude::*;

use rglk_graphics::{GraphicsBackend, SpriteColor};
use rglk_math::vectors::Vector2F;

mod assets;
mod errors;

pub struct MacroquadBackend {
    pub atlases: HashMap<String, assets::SpriteAtlas>,
    bounds: (Vec2, Vec2)
}
impl MacroquadBackend {
    pub fn new() -> Self {
        MacroquadBackend { atlases: HashMap::new(), bounds: (Vec2::ZERO, Vec2::ZERO) }
    }
    pub async fn load_atlas(
        &mut self,
        name: &str,
        path: &str,
        columns: u32,
        rows: u32,
        spacing: Option<f32>
    ) -> Result<(), errors::AssetError> {
        let atlas = assets::SpriteAtlas::new(
            path,
            columns,
            rows,
            spacing
        ).await?;
        self.atlases.insert(name.into(), atlas);
        Ok(())
    }
    pub fn set_bounds(&mut self, camera: &Camera2D) {
        let bounds_min = camera.screen_to_world(Vec2::new(0., 0.));
        let bounds_max = camera.screen_to_world(Vec2::new(screen_width(), screen_height()));
        self.bounds = (bounds_min, bounds_max);
    }
    fn draw_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        let Some(atlas) = self.atlases.get(atlas_name) else { return };
        let sprite = atlas.get_sprite(index);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(size.x, size.y)),
            source: Some(sprite),
            ..Default::default()
        };
        let macroquad_color = macroquad::color::Color::from_rgba(
            color.0,
            color.1,
            color.2,
            color.3
        );
        draw_texture_ex(atlas.tex, position.x, position.y, macroquad_color, params);
    }
}
impl GraphicsBackend for MacroquadBackend {
    fn draw_world_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        // draw only visible sprites
        if position.x > self.bounds.1.x || position.y > self.bounds.1.y { return };
        if position.x + size.x < self.bounds.0.x || position.y + size.y < self.bounds.0.y { return };
        self.draw_sprite(atlas_name, index, position, size, color);
    }
    fn draw_ui_sprite(
        &self,
        atlas_name: &str,
        index: u32,
        position: Vector2F,
        size: Vector2F,
        color: SpriteColor
    ) {
        self.draw_sprite(atlas_name, index, position, size, color);
    }
}
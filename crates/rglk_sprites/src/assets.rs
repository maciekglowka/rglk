use macroquad::prelude::*;
use std::collections::HashMap;

use super::errors::AssetError;

pub struct Assets {
    pub atlases: HashMap<String, SpriteAtlas>
}
impl Assets {
    pub fn new() -> Self {
        Assets { atlases: HashMap::new() }
    }
}

pub struct SpriteColor(pub u8, pub u8, pub u8, pub u8);
impl From<SpriteColor> for macroquad::color::Color {
    fn from(value: SpriteColor) -> Self {
        macroquad::color::Color::from_rgba(value.0, value.1, value.2, value.3)
    }
}

#[derive(Debug)]
pub struct SpriteAtlas {
    tex: Texture2D,
    columns: u32,
    rows: u32,
    grid_size: Vec2,
    grid_offset: Vec2,
    sprite_size: Vec2,
    // spacing: Option<f32>
}
impl SpriteAtlas {
    pub async fn new(
        path: &str,
        columns: u32,
        rows: u32,
        spacing: Option<f32>
    ) -> Result<Self, AssetError> {
        // TODO defer loading?
        let tex = load_texture(path).await
            .map_err(|_| AssetError(format!("Could not load {}", path)))?;
        tex.set_filter(FilterMode::Nearest);

        let grid_size = Vec2::new(
            tex.width() / columns as f32,
            tex.height() / rows as f32
        );
        let sprite_size = match spacing {
            Some(d) => Vec2::new(grid_size.x - d, grid_size.y - d),
            None => grid_size
        };
        let grid_offset = (grid_size - sprite_size) / 2.;

        Ok(SpriteAtlas {
            tex, sprite_size, grid_size, grid_offset, rows, columns
        })
    }
    pub fn draw_sprite(
        &self,
        pos: (f32, f32),
        size: (f32, f32),
        idx: u32,
        color: SpriteColor
    ) {
        let row = idx / self.columns;
        let col = idx % self.columns;
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(size.0, size.1)),
            source: Some(Rect::new(
                col as f32 * self.grid_size.x + self.grid_offset.x,
                row as f32 * self.grid_size.x + self.grid_offset.x,
                self.sprite_size.x,
                self.sprite_size.y
            )),
            ..Default::default()
        };
        draw_texture_ex(self.tex, pos.0, pos.1, color.into(), params);
    }
}
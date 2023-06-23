use rglk_sprites::{Assets, SpriteAtlas};

pub async fn load_assets(assets: &mut Assets) {
    let atlas = rglk_sprites::SpriteAtlas::new(
        "assets/sprites/ascii.png",
        16, 16,
        None
    ).await.unwrap();
    assets.atlases.insert("ascii".into(), atlas);
}
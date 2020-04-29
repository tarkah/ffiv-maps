use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::World,
    prelude::WorldExt,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteGrid, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
};

use std::path::Path;

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: impl AsRef<Path>,
    columns: u32,
    rows: u32,
    cell_size: (u32, u32),
) -> SpriteSheetHandle {
    let path = texture_path.as_ref().display().to_string();

    let texture = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(path, ImageFormat::default(), (), &texture_storage)
    };

    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &mut world.read_resource::<AssetStorage<SpriteSheet>>();

    let sprite_grid = SpriteGrid {
        texture_width: rows * cell_size.0,
        texture_height: columns * cell_size.1,
        columns,
        rows: Some(rows),
        sprite_count: Some(rows * columns),
        cell_size: Some(cell_size),
        ..Default::default()
    };

    let sprites = sprite_grid.build_sprites();
    loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}

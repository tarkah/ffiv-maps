use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::Transform,
    ecs::{prelude::World, Join},
    prelude::{Builder, GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteGrid, SpriteSheetHandle},
        SpriteSheet, Texture,
    },
    window::ScreenDimensions,
};

use std::{collections::HashMap, path::Path};

use crate::resources::{Map, TextureKind};

pub struct Game {
    pub load_map: Option<usize>,
    pub maps: Vec<&'static str>,
    pub current_map: usize,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            current_map: 0,
            load_map: None,
            maps: vec![
                "maps/castle1_baron_castle_01.ron",
                "maps/castle2_dwarven_01.ron",
                "maps/dtown_agart_01.ron",
                "maps/ship_adamant_forest_01.ron",
                "maps/town_mythril_01.ron",
            ],
        }
    }
}

#[derive(Default)]
pub struct LoadState {
    pub progress_counter: Option<ProgressCounter>,
    pub map_handle: Option<Handle<Map>>,
}

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.progress_counter = Some(ProgressCounter::default());

        self.map_handle = {
            let mut game = world.write_resource::<Game>();

            let map_idx = game.load_map.take().unwrap_or(0);
            let map_name = game.maps[map_idx];
            game.current_map = map_idx;

            dbg!((map_name, map_idx, game.current_map, game.load_map));

            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                map_name,
                RonFormat,
                self.progress_counter.as_mut().expect("map"),
                &world.read_resource::<AssetStorage<Map>>(),
            ))
        };

        load_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(ref progress_counter) = self.progress_counter {
            // Check if all data has been loaded
            if progress_counter.is_complete() {
                // Get the map, which is loaded in the on_start function of load state.
                let map = {
                    let map_storage = &data.world.read_resource::<AssetStorage<Map>>();
                    let map_handle = &self.map_handle.take().unwrap();
                    map_storage.get(map_handle).unwrap().clone()
                };

                let mut map_sheets = MapSpriteSheets::default();

                let base_sheet = get_sprite_sheet_handle(&data.world, &map.base, 16, 16);
                let var_sheet = get_sprite_sheet_handle(&data.world, &map.var, 16, 16);
                let anm_sheet = get_sprite_sheet_handle(&data.world, &map.anm, 16, 16);

                map_sheets.insert(TextureKind::Base, base_sheet);
                map_sheets.insert(TextureKind::Var, var_sheet);
                map_sheets.insert(TextureKind::Anm, anm_sheet);

                data.world.insert(map_sheets);

                resize_window_to_map(data.world, &map);

                map.load_map(data.world);

                self.progress_counter = None;
            }
        }

        let game = data.world.read_resource::<Game>();

        if game.load_map.is_some() {
            Trans::Replace(Box::new(LoadState::default()))
        } else {
            Trans::None
        }
    }
}

#[derive(Default)]
pub struct MapSpriteSheets {
    sprite_sheets: HashMap<TextureKind, SpriteSheetHandle>,
}

impl MapSpriteSheets {
    pub fn insert(&mut self, texture_kind: TextureKind, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(texture_kind, sprite_sheet_handle);
    }

    pub fn get(&self, texture_kind: TextureKind) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&texture_kind)
    }
}

fn load_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(width / 2.0, height / 2.0, 10.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: impl AsRef<Path>,
    width: usize,
    height: usize,
) -> SpriteSheetHandle {
    let path = texture_path.as_ref().display().to_string();

    let texture = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(path, ImageFormat::default(), (), &texture_storage)
    };

    let sprite_sheet_store = &mut world.write_resource::<AssetStorage<SpriteSheet>>();

    let sprite_grid = SpriteGrid {
        texture_width: width as u32 * 32,
        texture_height: height as u32 * 32,
        columns: width as u32,
        rows: Some(height as u32),
        sprite_count: Some((width * height) as u32),
        cell_size: Some((32, 32)),
        ..Default::default()
    };

    dbg!(&sprite_grid);

    let sprites = sprite_grid.build_sprites();
    sprite_sheet_store.insert(SpriteSheet { texture, sprites })
}

fn resize_window_to_map(world: &mut World, map: &Map) {
    let width = map.width as f32 * 32.0;
    let height = map.height as f32 * 32.0;

    {
        let mut dim = world.fetch_mut::<ScreenDimensions>();
        dim.update(width as f64, height as f64);
    }

    let mut transforms = world.write_storage::<Transform>();
    let mut cameras = world.write_storage::<Camera>();

    for (transform, camera) in (&mut transforms, &mut cameras).join() {
        transform.set_translation_xyz(width / 2.0, height / 2.0, 10.0);

        let projection = Projection::orthographic(
            -1.0 * width / 2.0,
            width / 2.0,
            -1.0 * height / 2.0,
            height / 2.0,
            0.1,
            2000.0,
        );
        camera.set_projection(projection);
    }
}

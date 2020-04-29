use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
};

use crate::components::camera::{center_camera_on_map, load_camera};
use crate::resources::{
    game::Game,
    map::{Map, MapSpriteSheets, TextureKind},
    sprites::get_sprite_sheet_handle,
};

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

                let base_sheet = get_sprite_sheet_handle(&data.world, &map.base, 16, 16, (32, 32));
                let var_sheet = get_sprite_sheet_handle(&data.world, &map.var, 16, 16, (32, 32));
                let anm_sheet = get_sprite_sheet_handle(&data.world, &map.anm, 16, 16, (32, 32));

                map_sheets.insert(TextureKind::Base, base_sheet);
                map_sheets.insert(TextureKind::Var, var_sheet);
                map_sheets.insert(TextureKind::Anm, anm_sheet);

                data.world.insert(map_sheets);

                center_camera_on_map(data.world, &map);

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

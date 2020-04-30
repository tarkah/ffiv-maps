use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::Time,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::{GameData, SimpleState, SimpleTrans, StateData, Trans, WorldExt},
};

use std::time::Duration;

use crate::entities::{
    camera::load_camera,
    camera_subject::{center_camera_subject, load_camera_subject},
    player_one::load_player_one,
};
use crate::resources::{
    asset::{load_assets, AssetType, PrefabList},
    game::Game,
    map::{Map, MapSpriteSheets, TextureKind},
    sprites::get_sprite_sheet_handle,
};
use crate::systems;

#[derive(Default)]
pub struct LoadState<'a, 'b> {
    pub progress_counter: Option<ProgressCounter>,
    pub map_handle: Option<Handle<Map>>,
    pub first_load: bool,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for LoadState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(systems::MapInputSystem, "map_input_system", &[])
            .with(
                systems::PlayerOneInputSystem,
                "player_one_input_system",
                &[],
            )
            .with(systems::CleanupSystem, "cleanup_system", &[])
            .with(
                systems::PlayerOneTransformationSystem,
                "player_one_transformation_system",
                &["player_one_input_system"],
            )
            .with(
                systems::CameraTransformationSystem,
                "camera_transformation_system",
                &["player_one_transformation_system"],
            )
            .with(
                systems::PlayerOneAnimationSystem,
                "player_one_animation_system",
                &["player_one_transformation_system"],
            )
            .with(
                systems::AnimationControlSystem,
                "animation_control_system",
                &["player_one_animation_system"],
            )
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        {
            let mut time = world.write_resource::<Time>();

            let fixed_duration = Duration::from_micros(41_666);
            time.set_fixed_time(fixed_duration);
        }

        self.progress_counter = if self.first_load {
            Some(load_assets(world, vec![AssetType::PlayerOne]))
        } else {
            Some(ProgressCounter::default())
        };

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

        let subject = load_camera_subject(world);
        load_camera(world, subject);
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

                {
                    let mut game = data.world.write_resource::<Game>();
                    game.map_width = map.width as f32 * 32.0;
                    game.map_height = map.height as f32 * 32.0;
                }

                let mut map_sheets = MapSpriteSheets::default();

                let base_sheet = get_sprite_sheet_handle(&data.world, &map.base, 16, 16, (32, 32));
                let var_sheet = get_sprite_sheet_handle(&data.world, &map.var, 16, 16, (32, 32));
                let anm_sheet = get_sprite_sheet_handle(&data.world, &map.anm, 16, 16, (32, 32));

                map_sheets.insert(TextureKind::Base, base_sheet);
                map_sheets.insert(TextureKind::Var, var_sheet);
                map_sheets.insert(TextureKind::Anm, anm_sheet);

                data.world.insert(map_sheets);

                center_camera_subject(data.world, &map);

                map.load_map(data.world);

                let player_one_prefab_handle = {
                    let prefab_list = data.world.read_resource::<PrefabList>();
                    prefab_list.get(AssetType::PlayerOne).unwrap().clone()
                };

                load_player_one(data.world, &map, player_one_prefab_handle);

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

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        Trans::None
    }
}

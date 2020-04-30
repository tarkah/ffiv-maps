use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::World,
};

use std::collections::HashMap;

use crate::components::animation::AnimationPrefabData;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    PlayerOne,
}

#[derive(Default)]
pub struct PrefabList {
    prefabs: HashMap<AssetType, Handle<Prefab<AnimationPrefabData>>>,
}

impl PrefabList {
    pub fn insert(
        &mut self,
        asset_type: AssetType,
        prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    ) {
        self.prefabs.insert(asset_type, prefab_handle);
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&Handle<Prefab<AnimationPrefabData>>> {
        self.prefabs.get(&asset_type)
    }
}

pub fn load_assets(world: &mut World, asset_type_list: Vec<AssetType>) -> ProgressCounter {
    let mut prefab_list = PrefabList::default();
    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let ron_path = match asset_type {
            AssetType::PlayerOne => "prefabs/character/cain.ron",
        };

        let handle = get_animation_prefab_handle(world, ron_path, &mut progress_counter);

        prefab_list.insert(asset_type, handle);
    }

    world.insert(prefab_list);
    progress_counter
}

/// Loads a `Prefab` with type `AnimationPrefabData` from the given path.
fn get_animation_prefab_handle(
    world: &mut World,
    ron_path: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<AnimationPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
        loader.load(ron_path, RonFormat, progress_counter)
    })
}

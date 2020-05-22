use amethyst::{
    assets::{Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    audio::{AT3Format, SourceHandle},
    ecs::prelude::{World, WorldExt},
};

use std::collections::HashMap;
use std::{iter::Cycle, vec::IntoIter};

use crate::components::animation::AnimationPrefabData;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    Character(Character),
    Audio(Audio),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Character {
    Cain,
    Cecil,
    Kyuucecil,
    Kyuurydia,
    Roza,
    Rydia,
    Yang,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Audio {
    BgmAi,
    BgmPrologue,
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

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

pub fn load_assets(world: &mut World, asset_type_list: &[AssetType]) -> ProgressCounter {
    let mut prefab_list = PrefabList::default();
    let mut music = vec![];

    let mut progress_counter = ProgressCounter::new();

    for &asset_type in asset_type_list.iter() {
        let path = match asset_type {
            AssetType::Character(character) => match character {
                Character::Cain => "prefabs/character/cain.ron",
                Character::Cecil => "prefabs/character/cecil.ron",
                Character::Kyuucecil => "prefabs/character/kyuucecil.ron",
                Character::Kyuurydia => "prefabs/character/kyuurydia.ron",
                Character::Roza => "prefabs/character/roza.ron",
                Character::Rydia => "prefabs/character/rydia.ron",
                Character::Yang => "prefabs/character/yang.ron",
            },
            AssetType::Audio(audio) => match audio {
                Audio::BgmAi => "audio/BGM_AI.at3",
                Audio::BgmPrologue => "audio/BGM_PROLOGUE.at3",
            },
        };

        match asset_type {
            AssetType::Character(_) => {
                let handle = get_animation_prefab_handle(world, path, &mut progress_counter);

                prefab_list.insert(asset_type, handle);
            }
            AssetType::Audio(_) => {
                let loader = world.read_resource::<Loader>();
                let handle: SourceHandle = loader.load(path, AT3Format, (), &world.read_resource());
                music.push(handle);
            }
        }
    }

    let music = Music {
        music: music.into_iter().cycle(),
    };

    world.insert(prefab_list);
    world.insert(music);

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

use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, Write},
    utils::removal::{exec_removal, Removal},
};

use crate::components::player_one::PlayerOne;
use crate::entities::player_one::reload_player_one;
use crate::resources::{asset::PrefabList, game::Game};

#[derive(SystemDesc)]
pub struct CleanupSystem;

impl<'s> System<'s> for CleanupSystem {
    type SystemData = (
        ReadStorage<'s, Removal<usize>>,
        Entities<'s>,
        Read<'s, Game>,
    );

    fn run(&mut self, (removals, entities, game): Self::SystemData) {
        if game.load_map.is_some() {
            exec_removal(&entities, &removals, 0);
        }
    }
}

#[derive(SystemDesc)]
pub struct PlayerOneReloadSystem;

impl<'s> System<'s> for PlayerOneReloadSystem {
    type SystemData = (
        Entities<'s>,
        Write<'s, Game>,
        Read<'s, PrefabList>,
        ReadStorage<'s, PlayerOne>,
        ReadStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (entities, mut game, prefab_list, player_one, transforms, updater): Self::SystemData,
    ) {
        for (entity, transform, _) in (&entities, &transforms, &player_one).join() {
            if game.load_char {
                let current_pos = (transform.translation().x, transform.translation().y);

                let _ = entities.delete(entity);

                let player_one_prefab_handle = {
                    prefab_list
                        .get(game.chars[game.current_char])
                        .unwrap()
                        .clone()
                };

                let player_one = entities.create();

                reload_player_one(
                    &updater,
                    player_one,
                    current_pos.0,
                    current_pos.1,
                    player_one_prefab_handle,
                );

                game.load_char = false;
            }
        }
    }
}

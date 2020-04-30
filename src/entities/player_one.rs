use amethyst::{
    assets::{Handle, Prefab},
    core::{Transform, WithNamed},
    ecs::prelude::World,
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
    utils::removal::Removal,
};

use crate::{
    components::{
        animation::{Animation, AnimationId, AnimationPrefabData, CharacterAction},
        direction::Direction,
        movement::Movement,
        player_one::PlayerOne,
    },
    resources::map::Map,
};

pub fn load_player_one(world: &mut World, map: &Map, prefab: Handle<Prefab<AnimationPrefabData>>) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(
        (map.width as f32 * 32.0) / 2.0 + 16.0,
        (map.height as f32 * 32.0) / 2.0 - 16.0,
        -1.0,
    );

    world
        .create_entity()
        .with(PlayerOne::new())
        .named("PlayerOne")
        .with(transform)
        .with(Animation::new(
            AnimationId::Character(CharacterAction::IdleForward),
            vec![
                AnimationId::Character(CharacterAction::IdleForward),
                AnimationId::Character(CharacterAction::IdleSideways),
                AnimationId::Character(CharacterAction::IdleBackward),
                AnimationId::Character(CharacterAction::RunForward),
                AnimationId::Character(CharacterAction::RunSideways),
                AnimationId::Character(CharacterAction::RunBackward),
            ],
        ))
        .with(prefab)
        .with(Transparent) // Necessary for ordered layering
        .with(Direction::default())
        .with(Movement::default())
        .with(Removal::new(0usize))
        .build();
}

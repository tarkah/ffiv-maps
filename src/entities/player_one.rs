use amethyst::{
    assets::{Handle, Prefab},
    core::Transform,
    ecs::prelude::{Entity, LazyUpdate, Read, World},
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

    let x_offset = if map.width % 2 == 0 { 16.0 } else { 0.0 };
    let y_offset = if map.height % 2 == 0 { 16.0 } else { 0.0 };

    transform.set_translation_xyz(
        (map.width as f32 * 32.0) / 2.0 + x_offset,
        (map.height as f32 * 32.0) / 2.0 + y_offset,
        -1.0,
    );

    world
        .create_entity()
        .with(PlayerOne::new())
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

pub fn reload_player_one(
    updater: &Read<LazyUpdate>,
    entity: Entity,
    x: f32,
    y: f32,
    prefab: Handle<Prefab<AnimationPrefabData>>,
) {
    let mut transform = Transform::default();

    transform.set_translation_xyz(x, y, -1.0);

    updater.insert(entity, PlayerOne::new());
    updater.insert(entity, transform);
    updater.insert(
        entity,
        Animation::new(
            AnimationId::Character(CharacterAction::IdleForward),
            vec![
                AnimationId::Character(CharacterAction::IdleForward),
                AnimationId::Character(CharacterAction::IdleSideways),
                AnimationId::Character(CharacterAction::IdleBackward),
                AnimationId::Character(CharacterAction::RunForward),
                AnimationId::Character(CharacterAction::RunSideways),
                AnimationId::Character(CharacterAction::RunBackward),
            ],
        ),
    );
    updater.insert(entity, prefab);
    updater.insert(entity, Transparent);
    updater.insert(entity, Direction::default());
    updater.insert(entity, Movement::default());
    updater.insert(entity, Removal::new(0usize));
}

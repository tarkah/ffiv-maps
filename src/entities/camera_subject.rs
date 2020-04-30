use amethyst::{
    core::Transform,
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
};

use crate::components::subject::Subject;

pub fn load_camera_subject(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 5.0);

    world
        .create_entity()
        .with(transform)
        .with(Subject::default())
        .with(Transparent)
        .build()
}

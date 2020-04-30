use amethyst::{
    core::Transform,
    ecs::{prelude::World, Entity, Join},
    prelude::{Builder, WorldExt},
    renderer::transparent::Transparent,
};

use crate::components::subject::Subject;
use crate::resources::map::Map;

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

pub fn center_camera_subject(world: &mut World, map: &Map) {
    let mut transforms = world.write_storage::<Transform>();
    let subjects = world.read_storage::<Subject>();

    for (transform, _) in (&mut transforms, &subjects).join() {
        transform.set_translation_xyz(
            (map.width as f32 * 32.0) / 2.0 + 16.0,
            (map.height as f32 * 32.0) / 2.0,
            10.0,
        );
    }
}

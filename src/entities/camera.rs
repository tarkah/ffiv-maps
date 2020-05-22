use amethyst::{
    core::{Parent, Transform},
    ecs::{prelude::World, Entity},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
};

pub fn load_camera(world: &mut World, camera_subject: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world
        .create_entity()
        .with(Camera::standard_2d(480.0, 272.0))
        .with(Parent {
            entity: camera_subject,
        })
        .with(transform)
        .build();
}

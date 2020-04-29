use amethyst::{
    core::Transform,
    ecs::{prelude::World, Join},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

use crate::resources::map::Map;

pub fn load_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    let mut transform = Transform::default();
    transform.set_translation_xyz(width / 2.0, height / 2.0, 10.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

pub fn center_camera_on_map(world: &mut World, map: &Map) {
    let mut transforms = world.write_storage::<Transform>();
    let cameras = world.read_storage::<Camera>();

    for (transform, _) in (&mut transforms, &cameras).join() {
        transform.set_translation_xyz(
            (map.width as f32 * 32.0) / 2.0,
            (map.height as f32 * 32.0) / 2.0,
            10.0,
        );
    }
}

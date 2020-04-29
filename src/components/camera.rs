use amethyst::{
    core::Transform,
    ecs::{prelude::World, Join},
    prelude::{Builder, WorldExt},
    renderer::camera::Camera,
    window::ScreenDimensions,
};

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

pub fn reset_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.fetch::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transforms = world.write_storage::<Transform>();
    let cameras = world.read_storage::<Camera>();

    for (transform, _) in (&mut transforms, &cameras).join() {
        transform.set_translation_xyz(width / 2.0, height / 2.0, 10.0);
    }
}

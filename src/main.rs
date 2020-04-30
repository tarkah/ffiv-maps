// Nearly every Amethyst system triggers this warning, better ignore it:
#![allow(clippy::type_complexity)]

use amethyst::{
    animation::AnimationBundle,
    assets::{PrefabLoaderSystemDesc, Processor},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::SpriteRender,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    Application, GameDataBuilder,
};

use components::animation::{AnimationId, AnimationPrefabData};
use resources::map::Map;

mod components;
mod entities;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let root = application_root_dir()?;
    let display_config_path = root.join("resources/display_config.ron");
    let bindings_path = root.join("resources/bindings.ron");
    let assets_path = root.join("assets");

    let prefab_loader_system_desc = PrefabLoaderSystemDesc::<AnimationPrefabData>::default();

    let game_data = GameDataBuilder::default()
        .with_system_desc(prefab_loader_system_desc, "scene_loader", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(TransformBundle::new().with_dep(&["sampler_interpolation_system"]))?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Map>::new(), "map_processor", &[]);

    let mut state = states::LoadState::default();
    state.first_load = true;

    let mut game = Application::build(assets_path, state)?.build(game_data)?;

    game.run();

    Ok(())
}

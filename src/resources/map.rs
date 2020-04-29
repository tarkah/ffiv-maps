use amethyst::{
    animation::{
        get_animation_set, Animation, AnimationCommand, AnimationControl, AnimationControlSet,
        ControlState, EndControl, InterpolationFunction, Sampler, SpriteRenderChannel,
        SpriteRenderPrimitive,
    },
    assets::{Asset, AssetStorage, Handle, ProcessingState},
    core::{
        math::{Vector2, Vector3},
        Transform, WithNamed,
    },
    ecs::{prelude::World, VecStorage},
    error::Error,
    prelude::{Builder, WorldExt},
    renderer::{sprite::SpriteRender, transparent::Transparent},
    utils::removal::Removal,
    window::ScreenDimensions,
};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::path::PathBuf;

use crate::states::MapSpriteSheets;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Eq, Hash)]
pub enum TextureKind {
    Base,
    Var,
    Anm,
}

impl Default for TextureKind {
    fn default() -> Self {
        TextureKind::Base
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum TriggerKind {
    Uninitialized,
    Passable,
    Blocker,
    UpperLowerDelta,
    LowerUpperDelta,
    Hidden,
    Bridge,
    Damage,
    BottomTransparent,
    BottomHidden,
    Unknown7,
    Unknown12,
    Unknown13,
    Treasure(u8),
    Exit(u8),
    Unknown(u8),
}

impl Default for TriggerKind {
    fn default() -> Self {
        TriggerKind::Uninitialized
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Cell {
    index: u8,
    kind: TextureKind,
    trigger: TriggerKind,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
struct Layer {
    cells: Vec<Cell>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub base: PathBuf,
    pub var: PathBuf,
    pub anm: PathBuf,
    layers: Vec<Layer>,
}

impl Asset for Map {
    const NAME: &'static str = "ffiv_maps::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Map>>;
}

impl Map {
    pub fn load_map(&self, world: &mut World) {
        for (idx, layer) in self.layers.iter().enumerate() {
            self.load_layer(world, layer, idx);
        }
    }

    fn load_layer(&self, world: &mut World, layer: &Layer, idx: usize) {
        let screen_height = {
            let dim = world.fetch::<ScreenDimensions>();
            dim.height()
        };

        let (base_sheet, var_sheet, anm_sheet) = {
            let map_sheets = world.read_resource::<MapSpriteSheets>();
            (
                map_sheets.get(TextureKind::Base).unwrap().clone(),
                map_sheets.get(TextureKind::Var).unwrap().clone(),
                map_sheets.get(TextureKind::Anm).unwrap().clone(),
            )
        };

        for (idx, cell) in layer.cells.iter().enumerate() {
            let sprite_sheet = match cell.kind {
                TextureKind::Base => base_sheet.clone(),
                TextureKind::Var => var_sheet.clone(),
                TextureKind::Anm => anm_sheet.clone(),
            };

            let render = SpriteRender {
                sprite_sheet,
                sprite_number: cell.index as usize,
            };

            let mut transform = Transform::default();
            transform.set_translation_xyz(
                (idx % self.width) as f32 * 32.0 + 16.0,
                screen_height - (((idx / self.width) as f32) * 32.0 + 16.0),
                -1.0 * idx as f32,
            );

            let entity = world
                .create_entity()
                .with(transform)
                .with(render)
                .with(Transparent)
                .with(Removal::new(0usize))
                .build();

            if cell.kind == TextureKind::Anm {
                let render_1 = SpriteRenderPrimitive::SpriteIndex(cell.index as usize);
                let render_2 = SpriteRenderPrimitive::SpriteIndex((cell.index + 1) as usize);
                let render_3 = SpriteRenderPrimitive::SpriteIndex((cell.index + 2) as usize);
                let render_4 = SpriteRenderPrimitive::SpriteIndex((cell.index + 3) as usize);

                let sampler = Sampler {
                    input: vec![0.0, 0.2, 0.4, 0.6],
                    output: vec![render_1, render_2, render_3, render_4],
                    function: InterpolationFunction::Step,
                };

                let sampler_handle = {
                    let sampler_store =
                        &mut world.write_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>();
                    sampler_store.insert(sampler)
                };

                let animation_hanle: Handle<Animation<SpriteRender>> = {
                    let animation =
                        Animation::new_single(0, SpriteRenderChannel::SpriteIndex, sampler_handle);
                    let animation_store =
                        &mut world.write_resource::<AssetStorage<Animation<SpriteRender>>>();
                    animation_store.insert(animation)
                };

                let mut storage = world.write_storage::<AnimationControlSet<usize, SpriteRender>>();
                let control_set = get_animation_set(&mut storage, entity).unwrap();

                control_set.add_animation(
                    0,
                    &animation_hanle,
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            };
        }
    }
}

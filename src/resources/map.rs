use amethyst::{
    animation::{
        get_animation_set, Animation, AnimationCommand, AnimationControlSet, EndControl,
        InterpolationFunction, Sampler, SpriteRenderChannel, SpriteRenderPrimitive,
    },
    assets::{Asset, AssetStorage, Handle},
    core::Transform,
    ecs::{prelude::World, VecStorage},
    prelude::{Builder, WorldExt},
    renderer::{
        sprite::{SpriteRender, SpriteSheetHandle},
        Transparent,
    },
    utils::removal::Removal,
};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, path::PathBuf};

use crate::components::{
    animation::AnimationId,
    map::{LowerTile, TriggerKind, UpperTile},
};

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

#[derive(Default)]
pub struct MapSpriteSheets {
    sprite_sheets: HashMap<TextureKind, SpriteSheetHandle>,
}

impl MapSpriteSheets {
    pub fn insert(&mut self, texture_kind: TextureKind, sprite_sheet_handle: SpriteSheetHandle) {
        self.sprite_sheets.insert(texture_kind, sprite_sheet_handle);
    }

    pub fn get(&self, texture_kind: TextureKind) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&texture_kind)
    }
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

    fn load_layer(&self, world: &mut World, layer: &Layer, layer_idx: usize) {
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
                (self.width as f32 * 32.0) - (((idx / self.width) as f32) * 32.0 + 16.0),
                2.0 * (layer_idx as f32 - 1.0),
            );

            let mut entity_builder = world
                .create_entity()
                .with(cell.trigger.clone())
                .with(transform)
                .with(render)
                .with(Transparent)
                .with(Removal::new(0usize));

            entity_builder = if layer_idx == 0 {
                entity_builder.with(LowerTile)
            } else {
                entity_builder.with(UpperTile)
            };

            let entity = entity_builder.build();

            if cell.kind == TextureKind::Anm {
                let render_1 = SpriteRenderPrimitive::SpriteIndex(cell.index as usize);
                let render_2 = SpriteRenderPrimitive::SpriteIndex((cell.index + 1) as usize);
                let render_3 = SpriteRenderPrimitive::SpriteIndex((cell.index + 2) as usize);
                let render_4 = SpriteRenderPrimitive::SpriteIndex((cell.index + 3) as usize);

                let sampler = Sampler {
                    input: vec![0.0, 0.25, 0.50, 0.75],
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

                let mut storage =
                    world.write_storage::<AnimationControlSet<AnimationId, SpriteRender>>();
                let control_set = get_animation_set(&mut storage, entity).unwrap();

                control_set.add_animation(
                    AnimationId::Map,
                    &animation_hanle,
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            };
        }
    }
}

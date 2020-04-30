use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::resources::Tint,
};

use crate::{
    components::map::{LowerTile, TriggerKind, UpperTile},
    resources::game::{DebugMode, Game},
};

#[derive(SystemDesc)]
pub struct DebugSystem;

impl<'s> System<'s> for DebugSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Tint>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>,
        ReadStorage<'s, UpperTile>,
        ReadStorage<'s, LowerTile>,
        ReadStorage<'s, TriggerKind>,
    );

    fn run(
        &mut self,
        (entities, mut tints, input, mut game, upper_tiles, lower_tiles, triggers): Self::SystemData,
    ) {
        if input.action_is_down("toggle_debug").unwrap_or(false) && !game.button_pressed {
            game.button_pressed = true;
            game.debug_mode = game.debug_mode.toggle();

            match game.debug_mode {
                DebugMode::Disabled => {
                    for (entity, _) in (&entities, &upper_tiles).join() {
                        tints.remove(entity);
                    }
                }
                DebugMode::TintLowerLayer => {
                    for (entity, _, trigger) in (&entities, &lower_tiles, &triggers).join() {
                        let _ = tints.insert(entity, trigger.tint());
                    }
                }
                DebugMode::TintUpperLayer => {
                    for (entity, _) in (&entities, &lower_tiles).join() {
                        tints.remove(entity);
                    }

                    for (entity, _, trigger) in (&entities, &upper_tiles, &triggers).join() {
                        let _ = tints.insert(entity, trigger.tint());
                    }
                }
            }
        }
    }
}

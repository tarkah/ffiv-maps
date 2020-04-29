use amethyst::derive::SystemDesc;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, SystemData, Write};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::utils::removal::{exec_removal, Removal};

use crate::states::Game;

#[derive(SystemDesc)]
pub struct CleanupSystem;

impl<'s> System<'s> for CleanupSystem {
    type SystemData = (
        ReadStorage<'s, Removal<usize>>,
        Entities<'s>,
        Read<'s, Game>,
    );

    fn run(&mut self, (removals, entities, game): Self::SystemData) {
        if game.load_map.is_some() {
            exec_removal(&entities, &removals, 0);
        }
    }
}

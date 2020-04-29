use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, Write};
use amethyst::input::{InputHandler, StringBindings};

use crate::states::Game;

#[derive(SystemDesc)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, Game>);

    fn run(&mut self, (input, mut game): Self::SystemData) {
        if input.action_is_down("next").unwrap_or(false) && game.load_map.is_none() {
            let idx = (game.current_map + 1) % game.maps.len();

            game.load_map = Some(idx);
        }

        if input.action_is_down("previous").unwrap_or(false) && game.load_map.is_none() {
            let idx = if game.current_map == 0 {
                game.maps.len() - 1
            } else {
                game.current_map - 1
            };

            game.load_map = Some(idx);
        }
    }
}

use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use crate::resources::game::Game;

#[derive(SystemDesc)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, mut game, cameras, mut transforms): Self::SystemData) {
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

        for (_, transform) in (&cameras, &mut transforms).join() {
            let scale = 6.0;

            if let Some(amount) = input.axis_value("up_down") {
                transform.prepend_translation_y(amount * scale);
            }

            if let Some(amount) = input.axis_value("left_right") {
                transform.prepend_translation_x(amount * scale);
            }
        }
    }
}

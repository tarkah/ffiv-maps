use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{
    direction::{Direction, Directions},
    player_one::{PlayerOne, PlayerOneState},
};
use crate::resources::game::Game;

#[derive(SystemDesc)]
pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Game>,
        WriteStorage<'s, PlayerOne>,
        WriteStorage<'s, Direction>,
    );

    fn run(&mut self, (input, mut game, mut player_one, mut directions): Self::SystemData) {
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

        for (player_one, direction) in (&mut player_one, &mut directions).join() {
            let mut x_amount = 0.0;
            let mut y_amount = 0.0;

            if let Some(amount) = input.axis_value("up_down") {
                y_amount = amount;
            }

            if let Some(amount) = input.axis_value("left_right") {
                x_amount = amount;
            }

            direction.previous = direction.current;

            if y_amount > 0.0 {
                direction.current = Directions::Up;
                player_one.state = PlayerOneState::Running;
            } else if y_amount < 0.0 {
                direction.current = Directions::Down;
                player_one.state = PlayerOneState::Running;
            } else if x_amount > 0.0 {
                direction.current = Directions::Right;
                player_one.state = PlayerOneState::Running;
            } else if x_amount < 0.0 {
                direction.current = Directions::Left;
                player_one.state = PlayerOneState::Running;
            } else {
                player_one.state = PlayerOneState::Idle;
            }
        }
    }
}

use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::{
    direction::{Direction, Directions},
    movement::Movement,
    player_one::{PlayerOne, PlayerOneState},
};
use crate::resources::game::Game;

#[derive(SystemDesc)]
pub struct MapInputSystem;

impl<'s> System<'s> for MapInputSystem {
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
#[derive(SystemDesc)]
pub struct PlayerOneInputSystem;

impl<'s> System<'s> for PlayerOneInputSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, PlayerOne>,
        WriteStorage<'s, Direction>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (input, mut player_one, mut directions, mut movements): Self::SystemData) {
        for (player_one, direction, movement) in
            (&mut player_one, &mut directions, &mut movements).join()
        {
            if movement.count == 0 {
                let x_amount = input.axis_value("left_right").unwrap_or(0.0);
                let y_amount = input.axis_value("up_down").unwrap_or(0.0);

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
}

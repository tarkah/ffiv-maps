use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::components::{
    direction::{Direction, Directions},
    player_one::{PlayerOne, PlayerOneState},
    subject::Subject,
};
use crate::resources::game::Game;

pub struct PlayerOneTransformationSystem;

impl<'s> System<'s> for PlayerOneTransformationSystem {
    type SystemData = (
        ReadStorage<'s, PlayerOne>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (player_one, directions, mut transforms): Self::SystemData) {
        for (player, direction, transform) in (&player_one, &directions, &mut transforms).join() {
            if player.state == PlayerOneState::Running {
                let run_amount = 4.0;

                match direction.current {
                    Directions::Up => {
                        transform.prepend_translation_y(run_amount);
                    }
                    Directions::Down => {
                        transform.prepend_translation_y(-run_amount);
                    }
                    Directions::Right => {
                        transform.prepend_translation_x(run_amount);
                    }
                    Directions::Left => {
                        transform.prepend_translation_x(-run_amount);
                    }
                }
            }

            if (direction.current == Directions::Right && direction.previous != Directions::Right)
                || (direction.current != Directions::Right
                    && direction.previous == Directions::Right)
            {
                transform.scale_mut().x = transform.scale().x * -1.0;
            }
        }
    }
}

pub struct CameraTransformationSystem;

impl<'s> System<'s> for CameraTransformationSystem {
    type SystemData = (
        ReadStorage<'s, PlayerOne>,
        ReadStorage<'s, Subject>,
        WriteStorage<'s, Transform>,
        Read<'s, Game>,
    );

    fn run(&mut self, (player_one, subjects, mut transforms, game): Self::SystemData) {
        let mut player_one_x = 0.0;
        let mut player_one_y = 0.0;

        let map_width = game.map_width;
        let map_height = game.map_height;

        for (_, transform) in (&player_one, &transforms).join() {
            player_one_x = transform.translation().x;
            player_one_y = transform.translation().y;
        }

        for (_, transform) in (&subjects, &mut transforms).join() {
            if player_one_x <= map_width && player_one_y <= map_height {
                transform.set_translation_x(player_one_x);
                transform.set_translation_y(player_one_y);
            }
        }
    }
}

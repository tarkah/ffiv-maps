use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{
        shred::Fetch, storage::MaskedStorage, Join, Read, ReadStorage, Storage, System, SystemData,
        Write, WriteStorage,
    },
    input::{InputHandler, StringBindings},
};

use crate::{
    components::{
        direction::{Direction, Directions},
        map::TriggerKind,
        movement::Movement,
        player_one::{PlayerOne, PlayerOneState},
    },
    resources::game::Game,
};

#[derive(SystemDesc)]
pub struct GeneralInputSystem;

impl<'s> System<'s> for GeneralInputSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, Game>);

    fn run(&mut self, (input, mut game): Self::SystemData) {
        if !game.button_pressed {
            if input.action_is_down("next_map").unwrap_or(false) && game.load_map.is_none() {
                game.button_pressed = true;

                let idx = (game.current_map + 1) % game.maps.len();

                game.load_map = Some(idx);
            }

            if input.action_is_down("previous_map").unwrap_or(false) && game.load_map.is_none() {
                game.button_pressed = true;

                let idx = if game.current_map == 0 {
                    game.maps.len() - 1
                } else {
                    game.current_map - 1
                };

                game.load_map = Some(idx);
            }

            if input.action_is_down("next_char").unwrap_or(false) && !game.load_char {
                game.button_pressed = true;

                let idx = (game.current_char + 1) % game.chars.len();

                game.load_char = true;
                game.current_char = idx;
            }

            if input.action_is_down("previous_char").unwrap_or(false) && !game.load_char {
                game.button_pressed = true;

                let idx = if game.current_char == 0 {
                    game.chars.len() - 1
                } else {
                    game.current_char - 1
                };

                game.load_char = true;
                game.current_char = idx;
            }
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
        ReadStorage<'s, Transform>,
        ReadStorage<'s, TriggerKind>,
    );

    fn run(
        &mut self,
        (input, mut player_one, mut directions, mut movements, transforms, triggers): Self::SystemData,
    ) {
        for (player_one, direction, movement, player_transform) in (
            &mut player_one,
            &mut directions,
            &mut movements,
            &transforms,
        )
            .join()
        {
            if movement.run_count == 0 {
                let movement_keys = ["n", "s", "e", "w"];
                let new_direction = movement_keys
                    .iter()
                    .filter_map(|s| {
                        if input.action_is_down(s.to_owned()).unwrap_or_default() {
                            Some(*s)
                        } else {
                            None
                        }
                    })
                    .map(Directions::from)
                    .next();

                if let Some(new_direction) = new_direction {
                    direction.previous = direction.current;
                    direction.current = new_direction;

                    // if new_direction == direction.previous
                    //     && player_one.state == PlayerOneState::Turning
                    //     && movement.turn_count != 0
                    // {
                    //     movement.turn_count = (movement.turn_count + 1) % 3;
                    //     return;
                    // }

                    // if player_one.state == PlayerOneState::Idle {
                    //     player_one.state = PlayerOneState::Turning;
                    //     movement.turn_count += 1;
                    // } else if is_blocker(player_transform, new_direction, &triggers, &transforms) {
                    //     player_one.state = PlayerOneState::Idle;
                    //     movement.turn_count = 0;
                    // } else if movement.turn_count == 0 {
                    //     player_one.state = PlayerOneState::Running;
                    // }

                    if is_blocker(player_transform, new_direction, &triggers, &transforms) {
                        player_one.state = PlayerOneState::Idle;
                    } else {
                        player_one.state = PlayerOneState::Running;
                    }
                } else {
                    player_one.state = PlayerOneState::Idle;
                    //     movement.turn_count = 0;
                }
            }
        }
    }
}

fn is_blocker(
    player_transform: &Transform,
    new_direction: Directions,
    triggers: &Storage<TriggerKind, Fetch<MaskedStorage<TriggerKind>>>,
    transforms: &Storage<Transform, Fetch<MaskedStorage<Transform>>>,
) -> bool {
    let new_position = match new_direction {
        Directions::North => (
            player_transform.translation().x,
            player_transform.translation().y + 32.0,
        ),
        Directions::South => (
            player_transform.translation().x,
            player_transform.translation().y - 32.0,
        ),
        Directions::East => (
            player_transform.translation().x + 32.0,
            player_transform.translation().y,
        ),
        Directions::West => (
            player_transform.translation().x - 32.0,
            player_transform.translation().y,
        ),
    };

    let mut inside_map = false;

    for (trigger, tile_transform) in (triggers, transforms).join() {
        let tile_position = (
            tile_transform.translation().x,
            tile_transform.translation().y,
        );

        if tile_position == new_position {
            inside_map = true;

            if trigger == &TriggerKind::Blocker {
                return true;
            }
        }
    }

    !inside_map
}

#[derive(SystemDesc)]
pub struct KeyReleaseSystem;

impl<'s> System<'s> for KeyReleaseSystem {
    type SystemData = (Read<'s, InputHandler<StringBindings>>, Write<'s, Game>);

    fn run(&mut self, (input, mut game): Self::SystemData) {
        if input.keys_that_are_down().peekable().peek().is_none() {
            game.button_pressed = false;
        }
    }
}

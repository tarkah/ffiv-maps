use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::{
    animation::{Animation, AnimationId, CharacterAction},
    direction::{Direction, Directions},
    player_one::{PlayerOne, PlayerOneState},
};

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animations, animation_sets, mut animation_control_sets) = data;

        // Iterate over all entities having Animation and AnimationSet components.
        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            // Fetch or create the AnimationControlSet for this entity.
            let animation_control_set =
                get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|&animation_id| {
                    // Add the animations to the AnimationControlSet if it doesn't exist already.
                    // This ensures they are re-added after a call to abort().
                    if !animation_control_set.has_animation(animation_id) {
                        animation_control_set.add_animation(
                            animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            EndControl::Loop(None),
                            1.0,
                            AnimationCommand::Init,
                        );
                    }
                });
            }

            // Start the animation for the current AnimationId
            animation_control_set.start(animation.current);
        }
    }
}

#[derive(Default)]
pub struct PlayerOneAnimationSystem;

impl<'s> System<'s> for PlayerOneAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, PlayerOne>,
        ReadStorage<'s, Direction>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_one, directions, mut animations, mut animation_control_sets) = data;

        for (_, player, direction, mut animation, animation_control_set) in (
            &entities,
            &player_one,
            &directions,
            &mut animations,
            &mut animation_control_sets,
        )
            .join()
        {
            let new_animation_id = match player.state {
                PlayerOneState::Idle => match direction.current {
                    Directions::North => AnimationId::Character(CharacterAction::IdleBackward),
                    Directions::South => AnimationId::Character(CharacterAction::IdleForward),
                    _ => AnimationId::Character(CharacterAction::IdleSideways),
                },
                PlayerOneState::Turning => match direction.current {
                    Directions::North => AnimationId::Character(CharacterAction::IdleBackward),
                    Directions::South => AnimationId::Character(CharacterAction::IdleForward),
                    _ => AnimationId::Character(CharacterAction::IdleSideways),
                },
                PlayerOneState::Running => match direction.current {
                    Directions::North => AnimationId::Character(CharacterAction::RunBackward),
                    Directions::South => AnimationId::Character(CharacterAction::RunForward),
                    _ => AnimationId::Character(CharacterAction::RunSideways),
                },
            };

            // If the new AnimationId is different to the current one, abort the
            // current animation and start the new one
            if animation.current != new_animation_id {
                animation_control_set.abort(animation.current);
                animation_control_set.start(new_animation_id);

                animation.current = new_animation_id;
            }
        }
    }
}

use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum PlayerOneState {
    Idle,
    Running,
}

impl Default for PlayerOneState {
    fn default() -> Self {
        PlayerOneState::Idle
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct PlayerOne {
    pub state: PlayerOneState,
}

impl PlayerOne {
    pub fn new() -> Self {
        PlayerOne {
            state: Default::default(),
        }
    }
}

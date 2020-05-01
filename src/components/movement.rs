use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Movement {
    pub run_count: u8,
    pub turn_count: u8,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            run_count: 0,
            turn_count: 0,
        }
    }
}

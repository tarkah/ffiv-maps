use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Movement {
    pub count: u8,
}

impl Default for Movement {
    fn default() -> Self {
        Self { count: 0 }
    }
}

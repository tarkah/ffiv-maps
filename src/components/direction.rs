use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Clone, Copy)]
pub enum Directions {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Direction {
    pub current: Directions,
    pub previous: Directions,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            current: Directions::Down,
            previous: Directions::Down,
        }
    }
}

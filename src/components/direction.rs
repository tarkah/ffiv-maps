use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Directions {
    North,
    South,
    East,
    West,
}

impl From<&str> for Directions {
    fn from(s: &str) -> Self {
        match s {
            "n" => Directions::North,
            "s" => Directions::South,
            "e" => Directions::East,
            "w" => Directions::West,
            _ => Directions::North,
        }
    }
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
            current: Directions::South,
            previous: Directions::South,
        }
    }
}

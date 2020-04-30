use amethyst::{
    ecs::{Component, DenseVecStorage, NullStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use serde::{Deserialize, Serialize};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct UpperTile;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct LowerTile;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Component)]
#[storage(DenseVecStorage)]
pub enum TriggerKind {
    Uninitialized,
    Passable,
    Blocker,
    UpperLowerDelta,
    LowerUpperDelta,
    Hidden,
    Bridge,
    Damage,
    BottomTransparent,
    BottomHidden,
    Unknown7,
    Unknown12,
    Unknown13,
    Treasure(u8),
    Exit(u8),
    Unknown(u8),
}

impl TriggerKind {
    pub fn tint(&self) -> Tint {
        match self {
            TriggerKind::Passable => Tint(Srgba::from_components((0.6, 1.0, 0.6, 1.0))),
            TriggerKind::Blocker => Tint(Srgba::from_components((1.0, 0.6, 0.6, 1.0))),
            _ => Tint(Srgba::from_components((1.0, 1.0, 1.0, 1.0))),
        }
    }
}

impl Default for TriggerKind {
    fn default() -> Self {
        TriggerKind::Uninitialized
    }
}

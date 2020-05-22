use crate::resources::asset::{AssetType, Audio, Character};

pub struct Game {
    pub load_map: Option<usize>,
    pub maps: [&'static str; 5],
    pub load_char: bool,
    pub chars: [AssetType; 7],
    pub music: [AssetType; 2],
    pub current_map: usize,
    pub current_char: usize,
    pub map_width: f32,
    pub map_height: f32,
    pub debug_mode: DebugMode,
    pub button_pressed: bool,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            current_map: 0,
            current_char: 0,
            load_map: None,
            maps: [
                "maps/castle1_baron_castle_01.ron",
                "maps/castle2_dwarven_01.ron",
                "maps/dtown_agart_01.ron",
                "maps/ship_adamant_forest_01.ron",
                "maps/town_mythril_01.ron",
            ],
            load_char: false,
            chars: [
                AssetType::Character(Character::Cain),
                AssetType::Character(Character::Cecil),
                AssetType::Character(Character::Kyuucecil),
                AssetType::Character(Character::Kyuurydia),
                AssetType::Character(Character::Roza),
                AssetType::Character(Character::Rydia),
                AssetType::Character(Character::Yang),
            ],
            music: [
                AssetType::Audio(Audio::BgmAi),
                AssetType::Audio(Audio::BgmPrologue),
            ],
            map_width: 0.0,
            map_height: 0.0,
            debug_mode: DebugMode::default(),
            button_pressed: false,
        }
    }
}

#[derive(Debug)]
pub enum DebugMode {
    Disabled,
    TintLowerLayer,
    TintUpperLayer,
}

impl DebugMode {
    pub fn toggle(&self) -> Self {
        match self {
            DebugMode::Disabled => DebugMode::TintLowerLayer,
            DebugMode::TintLowerLayer => DebugMode::TintUpperLayer,
            DebugMode::TintUpperLayer => DebugMode::Disabled,
        }
    }
}

impl Default for DebugMode {
    fn default() -> Self {
        DebugMode::Disabled
    }
}

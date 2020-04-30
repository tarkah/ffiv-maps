pub struct Game {
    pub load_map: Option<usize>,
    pub maps: [&'static str; 5],
    pub current_map: usize,
    pub map_width: f32,
    pub map_height: f32,
    pub debug_mode: DebugMode,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            current_map: 0,
            load_map: None,
            maps: [
                "maps/castle1_baron_castle_01.ron",
                "maps/castle2_dwarven_01.ron",
                "maps/dtown_agart_01.ron",
                "maps/ship_adamant_forest_01.ron",
                "maps/town_mythril_01.ron",
            ],
            map_width: 0.0,
            map_height: 0.0,
            debug_mode: DebugMode::default(),
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

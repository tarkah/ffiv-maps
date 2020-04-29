pub struct Game {
    pub load_map: Option<usize>,
    pub maps: [&'static str; 5],
    pub current_map: usize,
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
        }
    }
}

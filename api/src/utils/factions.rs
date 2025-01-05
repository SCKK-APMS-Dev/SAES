use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Factions {
    SCKK,
    TOW,
}

pub fn get_faction_id(faction: Factions) -> i8 {
    match faction {
        Factions::SCKK => 1,
        Factions::TOW => 3,
    }
}

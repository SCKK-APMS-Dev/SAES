use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Factions {
    SCKK,
    APMS,
    TOW,
}

pub fn get_faction_id(faction: Factions) -> i8 {
    match faction {
        Factions::SCKK => 1,
        Factions::APMS => 2,
        Factions::TOW => 3,
    }
}

// pub fn get_faction_by_id(id: i8) -> Factions {
//     match id {
//         1 => Factions::SCKK,
//         3 => Factions::TOW,
//         _ => Factions::SCKK,
//     }
// }

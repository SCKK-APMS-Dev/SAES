use serde::{Deserialize, Serialize};

use super::factions::Factions;

#[derive(Debug, Deserialize, Serialize)]
pub enum Permissions {
    SaesLogin,
    SaesMaintenance,
    SaesTest,
    SaesUcp(Factions),
    SaesSm(Factions),
    SaesFm(Factions),
}

pub fn get_perm(perm: Permissions) -> String {
    match perm {
        Permissions::SaesLogin => "saes.login".to_string(),
        Permissions::SaesMaintenance => "saes.maintenance".to_string(),
        Permissions::SaesSm(fact) => format!(
            "saes.sm.{}",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcp(fact) => format!(
            "saes.ucp.{}",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesFm(fact) => format!(
            "saes.fm.{}",
            match fact {
                Factions::SCKK => "taxi",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesTest => "saes.test".to_string(),
    }
}

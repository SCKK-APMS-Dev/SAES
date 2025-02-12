use serde::{Deserialize, Serialize};

use super::factions::Factions;

#[derive(Debug, Deserialize, Serialize)]
pub enum Permissions {
    SaesLogin,
    SaesMaintenance,
    SaesTest,
    SaesUcp(Factions),
    SaesAdmin(Factions),
    SaesAdminShift(Factions),
    SaesAdminFleet(Factions),
    SaesAdminFaction(Factions),
}

pub fn get_perm(perm: Permissions) -> String {
    match perm {
        Permissions::SaesLogin => "saes.login".to_string(),
        Permissions::SaesMaintenance => "saes.maintenance".to_string(),
        Permissions::SaesAdmin(fact) => format!(
            "saes.{}.admin",
            match fact {
                Factions::SCKK => "taxi",
                Factions::APMS => "apms",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesUcp(fact) => format!(
            "saes.{}.ucp",
            match fact {
                Factions::SCKK => "taxi",
                Factions::APMS => "apms",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesAdminShift(fact) => format!(
            "saes.{}.admin.shift",
            match fact {
                Factions::SCKK => "taxi",
                Factions::APMS => "apms",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesAdminFaction(fact) => format!(
            "saes.{}.admin.faction",
            match fact {
                Factions::SCKK => "taxi",
                Factions::APMS => "apms",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesAdminFleet(fact) => format!(
            "saes.{}.admin.fleet",
            match fact {
                Factions::SCKK => "taxi",
                Factions::APMS => "apms",
                Factions::TOW => "tow",
            }
        ),
        Permissions::SaesTest => "saes.test".to_string(),
    }
}

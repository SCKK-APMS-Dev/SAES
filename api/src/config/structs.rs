use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::factions::Factions;

#[derive(Debug, Serialize, Deserialize)]
pub enum ShiftAccess {
    SameShift,
    OtherManager,
    OtherShift,
}

impl Default for ShiftAccess {
    fn default() -> Self {
        Self::SameShift
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            maintenance: None,
            announcement: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionConfig {
    pub shift_access: ShiftAccess,
    pub access: FactionAccessConfig,
    pub site_access: FactionSiteAccessConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionAccessConfig {
    pub supplements: bool,
    pub hails: bool,
    pub bills: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionSiteAccessConfig {
    pub ucp: bool,
    pub admin: bool,
    pub shift: bool,
    pub fleet: bool,
    pub faction: bool,
}

impl Default for FactionConfig {
    fn default() -> Self {
        Self {
            shift_access: ShiftAccess::default(),
            access: FactionAccessConfig {
                supplements: true,
                hails: true,
                bills: true,
            },
            site_access: FactionSiteAccessConfig {
                ucp: true,
                admin: true,
                shift: true,
                fleet: true,
                faction: true,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub global: GlobalConfig,
    pub factions: HashMap<Factions, FactionConfig>,
}

impl Default for MainConfig {
    fn default() -> Self {
        let mut factions = HashMap::new();
        factions.insert(Factions::SCKK, FactionConfig::default());
        factions.insert(Factions::TOW, FactionConfig::default());
        factions.insert(Factions::APMS, FactionConfig::default());
        Self {
            global: GlobalConfig::default(),
            factions,
        }
    }
}

use serde::{Deserialize, Serialize};

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
    pub allow_ucp: bool,
    pub allow_shift: bool,
    pub allow_fleet: bool,
    pub allow_faction: bool,
    pub shift_access: ShiftAccess,
}

impl Default for FactionConfig {
    fn default() -> Self {
        Self {
            allow_faction: true,
            allow_fleet: true,
            allow_shift: true,
            allow_ucp: true,
            shift_access: ShiftAccess::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub global: GlobalConfig,
    pub taxi: FactionConfig,
    pub tow: FactionConfig,
}

impl Default for MainConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            taxi: FactionConfig::default(),
            tow: FactionConfig::default(),
        }
    }
}

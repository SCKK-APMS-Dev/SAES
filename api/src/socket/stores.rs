use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct SocketStores {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

pub fn get_stores() -> SocketStores {
    let mut maintenance = File::open("stores/maintenance.store")
        .expect("Maintenance.store fájl megnyitása sikertelen");
    let mut announcement = File::open("stores/announcement.store")
        .expect("announcement.store fájl megnyitása sikertelen");
    let mut maintenance_string = String::new();
    let mut announcement_string = String::new();
    maintenance
        .read_to_string(&mut maintenance_string)
        .expect("String alakítás sikertelen");
    announcement
        .read_to_string(&mut announcement_string)
        .expect("String alakítás sikertelen");
    SocketStores {
        maintenance: Some(maintenance_string),
        announcement: Some(announcement_string),
    }
}

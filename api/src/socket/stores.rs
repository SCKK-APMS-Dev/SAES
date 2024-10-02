use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct SocketStores {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

pub fn get_stores() -> SocketStores {
    let stores_dir = Path::new("stores");
    if stores_dir.exists() == false {
        fs::create_dir(stores_dir).expect("Mappa létrehozása sikertelen");
    }
    let maintenance_path = Path::new("stores/maintenance.store");
    if maintenance_path.exists() == false {
        File::create(maintenance_path).expect("maintenance.store létrehozása sikertelen");
    }
    let announcement_path = Path::new("stores/announcement.store");
    if announcement_path.exists() == false {
        File::create(announcement_path).expect("announcement.store létrehozása sikertelen");
    }
    let mut maintenance = File::open(maintenance_path).unwrap();
    let mut announcement = File::open(announcement_path).unwrap();
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

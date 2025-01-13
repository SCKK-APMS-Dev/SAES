use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::BASE_HASHMAP;

#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct SocketStores {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

pub async fn get_stores() -> SocketStores {
    let mut maintenance = File::open("stores/maintenance.store").unwrap();
    let mut announcement = File::open("stores/announcement.store").unwrap();
    let mut maintenance_string = String::new();
    let mut announcement_string = String::new();
    maintenance
        .read_to_string(&mut maintenance_string)
        .expect("String alakítás sikertelen");
    announcement
        .read_to_string(&mut announcement_string)
        .expect("String alakítás sikertelen");
    let mut hash = BASE_HASHMAP.write().await;
    hash.insert("store_maintenance".to_string(), maintenance_string.clone());
    hash.insert(
        "store_announcement".to_string(),
        announcement_string.clone(),
    );
    SocketStores {
        maintenance: Some(maintenance_string),
        announcement: Some(announcement_string),
    }
}

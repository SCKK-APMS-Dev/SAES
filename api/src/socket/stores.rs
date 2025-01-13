use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::BASE_HASHMAP;

#[derive(Debug, Deserialize, Iterable, Serialize)]
pub struct SocketStores {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

pub async fn get_stores() -> SocketStores {
    let hash = BASE_HASHMAP.read().await;
    let maintenance_string = hash.get("store_maintenance").unwrap().to_string();
    let announcement_string = hash.get("store_announcement").unwrap().to_string();
    SocketStores {
        maintenance: Some(maintenance_string),
        announcement: Some(announcement_string),
    }
}

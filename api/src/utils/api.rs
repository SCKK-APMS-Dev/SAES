use crate::BASE_HASHMAP;

pub struct Apis {
    pub samt: String,
    pub sckkapp: String,
}

pub async fn get_api_envs() -> Apis {
    let hash = BASE_HASHMAP.read().await;
    let samt = hash.get("api_samt").unwrap();
    let sckkapp = hash.get("api_sckkapp").unwrap();
    Apis {
        samt: samt.to_owned(),
        sckkapp: sckkapp.to_owned(),
    }
}

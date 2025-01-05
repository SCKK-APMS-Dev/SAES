use std::env;

pub struct Apis {
    pub samt: String,
    pub sckkapp: String,
}

pub fn get_api_envs() -> Apis {
    let samt =
        env::var("SAMT_API").expect("SAMT_API .env fájlból betöltése sikertelen. Létre van hozva?");
    let sckkapp = env::var("SCKKAPP_API")
        .expect("SCKKAPP_API .env fájlból betöltése sikertelen. Létre van hozva?");
    Apis { samt, sckkapp }
}

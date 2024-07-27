use std::env;

pub struct Apis {
    pub patrik: String,
    pub erik: String,
}

pub fn get_api_envs() -> Apis {
    let patrik = env::var("PATRIK_API")
        .expect("PATRIK_API .env fájlból betöltése sikertelen. Létre van hozva?");
    let erik =
        env::var("ERIK_API").expect("ERIK_API .env fájlból betöltése sikertelen. Létre van hozva?");
    Apis { patrik, erik }
}

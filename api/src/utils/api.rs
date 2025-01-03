use std::env;

pub struct Apis {
    pub samt: String,
    pub sckkapp: String,
    pub testpass: Option<String>,
}

pub fn get_api_envs() -> Apis {
    let samt =
        env::var("SAMT_API").expect("SAMT_API .env fájlból betöltése sikertelen. Létre van hozva?");
    let sckkapp = env::var("SCKKAPP_API")
        .expect("SCKKAPP_API .env fájlból betöltése sikertelen. Létre van hozva?");
    let testpass = env::var("TEST_API_PASS");
    Apis {
        samt,
        sckkapp,
        testpass: testpass.ok(),
    }
}

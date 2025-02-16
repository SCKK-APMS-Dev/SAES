use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde_json::to_string_pretty;

use crate::MAIN_CONFIG;

use super::structs::MainConfig;

pub fn check_config() {
    let dir = Path::new("./config");
    if !dir.exists() {
        fs::create_dir(dir).unwrap();
    }
    let config_file = Path::new("./config/main.json");
    if !config_file.exists() {
        let mut file = File::create(config_file).unwrap();
        let config = MainConfig::default();
        let json = to_string_pretty(&config).unwrap();
        let buf = json.as_bytes();
        file.write(buf).unwrap();
    }
    let config = File::open(config_file);
    if config.is_err() {
        panic!("Config load failed.")
    }
    let config: Result<MainConfig, serde_json::Error> = serde_json::from_reader(config.unwrap());
    if config.is_err() {
        panic!("Config parse failed.")
    }
    MAIN_CONFIG.set(config.unwrap()).unwrap();
}

pub fn write_config(config: &MainConfig) {
    let config_file = Path::new("./config/main.json");
    let mut file = File::open(config_file).unwrap();
    let json = to_string_pretty(config).unwrap();
    let buf = json.as_bytes();
    file.write(buf).unwrap();
    check_config();
}

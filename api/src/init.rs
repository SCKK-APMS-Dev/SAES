use std::{
    fs::{self, File},
    path::Path,
};

fn stores_init() {
    let stores_dir = Path::new("stores");
    if !stores_dir.exists() {
        fs::create_dir(stores_dir).expect("Store dir létrehozása sikertelen");
    }
    let maintenance_path = Path::new("stores/maintenance.store");
    if maintenance_path.exists() == false {
        File::create(maintenance_path).expect("maintenance.store létrehozása sikertelen");
    }
    let announcement_path = Path::new("stores/announcement.store");
    if announcement_path.exists() == false {
        File::create(announcement_path).expect("announcement.store létrehozása sikertelen");
    }
}

fn image_init() {
    let image_dir = Path::new("public");
    if !image_dir.exists() {
        fs::create_dir(image_dir).expect("Image dir létrehozása sikertelen");
    }
}

fn image_tmp_init() {
    let image_tmp_dir = Path::new("public/tmp");
    if !image_tmp_dir.exists() {
        fs::create_dir(image_tmp_dir).expect("image_tmp_dir létrehozása sikertelen")
    }
}

pub fn main() {
    stores_init();
    image_init();
    image_tmp_init();
}

use std::{env, fs, path::Path, process::Command, thread, time::Duration};

use dotenvy::dotenv;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, SelectColumns, Set};
use utils::{ffmpeg::get_ffmpeg, sql::get_conn};

use db::data::{self as Data, ActiveModel, Model};

mod db;
mod init;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    init::main();
    let dir = env::var("CONVERT_DIR").expect("CONVERT_DIR lekérdezése sikertelen");
    let db = get_conn().await;
    loop {
        let data = Data::Entity::find()
            .select_column(Data::Column::Kep)
            .filter(Data::Column::Type.ne("leintés"))
            .all(&db)
            .await
            .expect("Adatbázis lekérés sikertelen");
        let ffmpeg = get_ffmpeg();
        convert(data, ffmpeg, &dir, &db).await;
        println!("===== DONE =====");
        thread::sleep(Duration::from_secs(60 * 60 * 1));
    }
}
async fn convert(modl: Vec<Model>, ffmpeg: String, dir: &String, db: &DatabaseConnection) {
    for item in modl.iter() {
        if !Path::new(&format!("error/{}", item.id)).exists()
            && !Path::new(&format!("error/db-{}", item.id)).exists()
        {}
        if item.r#type != "leintés" {
            if !item.kep.ends_with(".avif") {
                let kep_rename = item.kep.split(".").collect::<Vec<&str>>();
                let mut kep_rebuilt: String;
                if kep_rename.len() > 2 {
                    let mut vect = Vec::new();
                    for kep in &kep_rename {
                        if kep_rename.clone().iter().position(|n| n == kep).unwrap()
                            != kep_rename.len() - 1
                        {
                            vect.push(kep);
                        }
                    }
                    kep_rebuilt = format!(
                        "{}.avif",
                        vect.iter().map(|s| s.to_string()).collect::<String>()
                    );
                } else {
                    kep_rebuilt = format!("{}.avif", kep_rename[0]);
                }
                if kep_rebuilt.starts_with("tmp/") {
                    kep_rebuilt.remove(0); // remove t
                    kep_rebuilt.remove(0); // remove m
                    kep_rebuilt.remove(0); // remove p
                    kep_rebuilt.remove(0); // remove /
                }
                let convert = Command::new(ffmpeg.clone())
                    .arg("-y")
                    .arg("-i")
                    .arg(format!("{}/{}", dir, item.kep))
                    .arg(format!("{}/{}", dir, kep_rebuilt))
                    .spawn()
                    .expect("ffmpeg nem sikerült")
                    .wait_with_output();
                if convert.unwrap().status.code().unwrap() == 0 {
                    let activem = ActiveModel {
                        id: Set(item.id),
                        kep: Set(kep_rebuilt),
                        ..Default::default()
                    };
                    let dbupdate = Data::Entity::update(activem).exec(db).await;
                    if dbupdate.is_ok() {
                        fs::remove_file(format!("{}/{}", dir, item.kep))
                            .expect("Fájltörlés sikertelen");
                    } else {
                        fs::write(format!("error/db-{}", item.id), "")
                            .expect("error db lementése sikertelen");
                    }
                } else {
                    fs::write(format!("error/{}", item.id), "")
                        .expect("error lementése sikertelen");
                }
            }
        }
    }
}

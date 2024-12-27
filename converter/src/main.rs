use std::{env, fs, path::Path, process::Command, thread, time::Duration};

use chrono::{Timelike, Utc};
use dotenvy::dotenv;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use utils::{ffmpeg::get_ffmpeg, sql::get_db_conn};

use db::images::{self as Images, ActiveModel, Model};

mod db;
mod init;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    init::main();
    let dir = env::var("CONVERT_DIR").expect("CONVERT_DIR lekérdezése sikertelen");
    let db = get_db_conn().await;
    loop {
        let data = Images::Entity::find()
            .filter(Images::Column::Converted.eq(0))
            .all(&db)
            .await
            .expect("Adatbázis lekérés sikertelen");
        let ffmpeg = get_ffmpeg();
        if (Utc::now().hour() < 20 && Utc::now().hour() >= 8) || env::var("TIME_BYPASS").is_ok() {
            convert(data, ffmpeg, &dir, &db).await;
            println!("===== DONE =====");
        } else {
            println!("===== NINCS CONVERT, PATRIK ALSZIK =====");
        }
        thread::sleep(Duration::from_secs(60 * 60 * 1));
    }
}
async fn convert(modl: Vec<Model>, ffmpeg: String, dir: &String, db: &DatabaseConnection) {
    for item in modl.iter() {
        if !Path::new(&format!("error/{}", item.id)).exists()
            && !Path::new(&format!("error/db-{}", item.id)).exists()
        {
            let kep_rename = item.filename.split(".").collect::<Vec<&str>>();
            let kep_rebuilt: String;
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
            if Path::new(&format!(
                "{}/{}",
                if item.tmp == 1 {
                    dir.to_owned() + "/tmp"
                } else {
                    dir.to_owned()
                },
                item.filename
            ))
            .exists()
            {
                let convert = Command::new(ffmpeg.clone())
                    .arg("-y")
                    .arg("-i")
                    .arg(format!(
                        "{}/{}",
                        if item.tmp == 1 {
                            dir.to_owned() + "/tmp"
                        } else {
                            dir.to_owned()
                        },
                        item.filename
                    ))
                    .arg(format!("{}/{}", dir, kep_rebuilt))
                    .spawn()
                    .expect("ffmpeg nem sikerült")
                    .wait_with_output();
                if convert.unwrap().status.code().unwrap() == 0 {
                    let activem = ActiveModel {
                        id: Set(item.id),
                        filename: Set(kep_rebuilt.clone()),
                        tmp: Set(0),
                        converted: Set(1),
                        ..Default::default()
                    };
                    let dbupdate = Images::Entity::update(activem).exec(db).await;
                    if dbupdate.is_ok() {
                        fs::remove_file(format!(
                            "{}/{}",
                            if item.tmp == 1 {
                                dir.to_owned() + "/tmp"
                            } else {
                                dir.to_owned()
                            },
                            item.filename
                        ))
                        .expect("Fájltörlés sikertelen");
                    } else {
                        fs::write(
                            format!("error/db-{}", item.id),
                            format!("{} ---> {}", item.filename, kep_rebuilt),
                        )
                        .expect("error db lementése sikertelen");
                    }
                } else {
                    fs::write(
                        format!("error/{}", item.id),
                        format!("{} ---> {}", item.filename, kep_rebuilt),
                    )
                    .expect("error lementése sikertelen");
                }
            } else if Path::new(&format!("{}/{}", dir, kep_rebuilt)).exists() {
                println!("{} már konvertálva, db-be átírás", item.id);
                let activem = ActiveModel {
                    id: Set(item.id),
                    filename: Set(kep_rebuilt.clone()),
                    tmp: Set(0),
                    converted: Set(1),
                    ..Default::default()
                };
                Images::Entity::update(activem)
                    .exec(db)
                    .await
                    .expect("Meglévő átírása db-be sikertelen");
                println!("{} kész", item.id);
            } else {
                fs::write(
                    format!("error/{}", item.id),
                    format!("{} ---> {}", item.filename, kep_rebuilt),
                )
                .expect("error lementése sikertelen");
            }
        }
    }
}

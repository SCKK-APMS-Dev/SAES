use std::{env, fs, path::Path, process::Command, thread, time::Duration};

use chrono::{Timelike, Utc};
use dotenvy::dotenv;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
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
            .all(&db)
            .await
            .expect("Adatbázis lekérés sikertelen");
        let ffmpeg = get_ffmpeg();
        if Utc::now().hour() < 20 && Utc::now().hour() >= 8 {
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
                    if Path::new(&format!("{}/{}", dir, item.kep)).exists() {
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
                                kep: Set(kep_rebuilt.clone()),
                                ..Default::default()
                            };
                            let dbupdate = Data::Entity::update(activem).exec(db).await;
                            if dbupdate.is_ok() {
                                fs::remove_file(format!("{}/{}", dir, item.kep))
                                    .expect("Fájltörlés sikertelen");
                            } else {
                                fs::write(
                                    format!("error/db-{}", item.id),
                                    format!("{} ---> {}", item.kep, kep_rebuilt),
                                )
                                .expect("error db lementése sikertelen");
                            }
                        } else {
                            fs::write(
                                format!("error/{}", item.id),
                                format!("{} ---> {}", item.kep, kep_rebuilt),
                            )
                            .expect("error lementése sikertelen");
                        }
                    } else if Path::new(&format!("{}/{}", dir, kep_rebuilt)).exists() {
                        println!("{} már konvertálva, db-be átírás", item.id);
                        let activem = ActiveModel {
                            id: Set(item.id),
                            kep: Set(kep_rebuilt.clone()),
                            ..Default::default()
                        };
                        Data::Entity::update(activem)
                            .exec(db)
                            .await
                            .expect("Meglévő átírása db-be sikertelen");
                        println!("{} kész", item.id);
                    } else {
                        fs::write(
                            format!("error/{}", item.id),
                            format!("{} ---> {}", item.kep, kep_rebuilt),
                        )
                        .expect("error lementése sikertelen");
                    }
                }
            } else {
                let arraj = item.kep.split_once(",").unwrap();
                let mut first_elem = arraj.0.to_string();
                first_elem.pop();
                first_elem = first_elem.chars().skip(2).collect();
                let mut second_elem = arraj.1.to_string();
                second_elem.pop();
                second_elem.pop();
                second_elem = second_elem.chars().skip(1).collect();
                let first_kep_rename = first_elem.split(".").collect::<Vec<&str>>();
                let mut first_kep_rebuilt: String;
                let mut needdb_update = false;
                if first_kep_rename.len() > 2 {
                    let mut vect = Vec::new();
                    for kep in &first_kep_rename {
                        if first_kep_rename
                            .clone()
                            .iter()
                            .position(|n| n == kep)
                            .unwrap()
                            != first_kep_rename.len() - 1
                        {
                            vect.push(kep);
                        }
                    }
                    first_kep_rebuilt = format!(
                        "{}.avif",
                        vect.iter().map(|s| s.to_string()).collect::<String>()
                    );
                } else {
                    first_kep_rebuilt = format!("{}.avif", first_kep_rename[0]);
                }
                if first_kep_rebuilt.starts_with("tmp/") {
                    first_kep_rebuilt.remove(0); // remove t
                    first_kep_rebuilt.remove(0); // remove m
                    first_kep_rebuilt.remove(0); // remove p
                    first_kep_rebuilt.remove(0); // remove /
                }

                let second_kep_rename = second_elem.split(".").collect::<Vec<&str>>();
                let mut second_kep_rebuilt: String;
                if second_kep_rename.len() > 2 {
                    let mut vect = Vec::new();
                    for kep in &second_kep_rename {
                        if second_kep_rename
                            .clone()
                            .iter()
                            .position(|n| n == kep)
                            .unwrap()
                            != second_kep_rename.len() - 1
                        {
                            vect.push(kep);
                        }
                    }
                    second_kep_rebuilt = format!(
                        "{}.avif",
                        vect.iter().map(|s| s.to_string()).collect::<String>()
                    );
                } else {
                    second_kep_rebuilt = format!("{}.avif", second_kep_rename[0]);
                }
                if second_kep_rebuilt.starts_with("tmp/") {
                    second_kep_rebuilt.remove(0); // remove t
                    second_kep_rebuilt.remove(0); // remove m
                    second_kep_rebuilt.remove(0); // remove p
                    second_kep_rebuilt.remove(0); // remove /
                }
                if !first_elem.ends_with(".avif") {
                    if Path::new(&format!("{}/{}", dir, first_elem)).exists() {
                        let convert = Command::new(ffmpeg.clone())
                            .arg("-y")
                            .arg("-i")
                            .arg(format!("{}/{}", dir, first_elem))
                            .arg(format!("{}/{}", dir, first_kep_rebuilt))
                            .spawn()
                            .expect("ffmpeg nem sikerült")
                            .wait_with_output();
                        if convert.unwrap().status.code().unwrap() != 0 {
                            fs::write(
                                format!("error/{}", item.id),
                                format!("{} ---> {}", first_elem, first_kep_rebuilt),
                            )
                            .expect("error lementése sikertelen");
                            continue;
                        }
                        needdb_update = true;
                    } else if Path::new(&format!("{}/{}", dir, first_kep_rebuilt)).exists() {
                        println!("{} leintés 0 már konvertálva", item.id);
                        needdb_update = true;
                    } else {
                        fs::write(
                            format!("error/{}", item.id),
                            format!("notfound: {} ---> {}", first_elem, first_kep_rebuilt),
                        )
                        .expect("error lementése sikertelen");
                        continue;
                    }
                }
                if !second_elem.ends_with(".avif") {
                    if Path::new(&format!("{}/{}", dir, second_elem)).exists() {
                        let convert = Command::new(ffmpeg.clone())
                            .arg("-y")
                            .arg("-i")
                            .arg(format!("{}/{}", dir, second_elem))
                            .arg(format!("{}/{}", dir, second_kep_rebuilt))
                            .spawn()
                            .expect("ffmpeg nem sikerült")
                            .wait_with_output();
                        if convert.unwrap().status.code().unwrap() == 0 {
                            needdb_update = true;
                        } else {
                            fs::write(
                                format!("error/{}", item.id),
                                format!("{} ---> {}", second_elem, second_kep_rebuilt),
                            )
                            .expect("error lementése sikertelen");
                            continue;
                        }
                    } else if Path::new(&format!("{}/{}", dir, second_kep_rebuilt)).exists() {
                        println!("{} leintés 1 konvertálva, db-be átírás", item.id);
                        needdb_update = true;
                    } else {
                        fs::write(
                            format!("error/{}", item.id),
                            format!("notfound: {} ---> {}", second_elem, second_kep_rebuilt),
                        )
                        .expect("error lementése sikertelen");
                        continue;
                    }
                }
                if needdb_update {
                    let activem = ActiveModel {
                        id: Set(item.id),
                        kep: Set(format!(
                            "['{}','{}']",
                            first_kep_rebuilt, second_kep_rebuilt
                        )),
                        ..Default::default()
                    };
                    Data::Entity::update(activem).exec(db).await.unwrap();
                }
            }
        }
    }
}

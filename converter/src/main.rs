use dotenvy::dotenv;
use saes_api::db::data as Data;
use sea_orm::{EntityTrait, SelectColumns};
use utils::sql::get_conn;

mod utils;

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl nem található");
    let db = get_conn().await;
    let data = Data::Entity::find()
        .select_column(Data::Column::Kep)
        .all(&db)
        .await
        .expect("Adatbázis lekérés sikertelen");
    println!("{:?}", data)
}

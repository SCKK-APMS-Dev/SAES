use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeQuery {
    pub tipus: String,
}

#[derive(Debug, Deserialize)]
pub struct TypeExtraQuery {
    pub tipus: String,
    pub dates: String,
}

#[derive(Debug, Deserialize)]
pub struct ImgQuery {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ImgLeintQuery {
    pub id: String,
    pub ver: String,
}

#[derive(Debug, Deserialize)]
pub struct StatQuery {
    pub week: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminItemsQuery {
    pub tipus: String,
    pub status: String,
}

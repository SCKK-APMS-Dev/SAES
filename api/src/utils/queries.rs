use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BaseImgQuery {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseImgLeintQuery {
    pub id: String,
    pub ver: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseListQuery {
    pub tipus: String,
    pub driver: String,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeQuery {
    pub tipus: String,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeExtraQuery {
    pub tipus: String,
    pub dates: String,
}

#[derive(Debug, Deserialize)]
pub struct MVStatQuery {
    pub week: String,
}

#[derive(Debug, Deserialize)]
pub struct MVItemsQuery {
    pub tipus: String,
    pub status: String,
}

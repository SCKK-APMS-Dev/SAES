use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BaseImgQuery {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseListQuery {
    pub tipus: String,
    pub driver: String,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeQuery {
    pub tipus: i8,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeExtraQuery {
    pub tipus: i8,
    pub dates: String,
}

#[derive(Debug, Deserialize)]
pub struct MVStatQuery {
    pub week: String,
}

#[derive(Debug, Deserialize)]
pub struct MVItemsQuery {
    pub tipus: i8,
    pub status: i8,
}

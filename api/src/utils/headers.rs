use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeHeader {
    pub tipus: String,
}

#[derive(Debug, Deserialize)]
pub struct TypeExtraHeader {
    pub tipus: String,
    pub dates: String,
}

#[derive(Debug, Deserialize)]
pub struct ImgHeader {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ImgLeintHeader {
    pub id: String,
    pub ver: String,
}

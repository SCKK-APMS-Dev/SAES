use crate::db::{
    hailes::{
        ActiveModel as H_ActiveModel, Column as H_Column, Entity as H_Entity, Model as H_Model,
    },
    service_bills::{
        ActiveModel as SB_ActiveModel, Column as SB_Column, Entity as SB_Entity, Model as SB_Model,
    },
    supplements::{
        ActiveModel as S_ActiveModel, Column as S_Column, Entity as S_Entity, Model as S_Model,
    },
};

#[derive(Debug)]
pub struct Potlek {
    pub entity: S_Entity,
    pub model: S_Model,
    pub amodel: S_ActiveModel,
    pub column: S_Column,
}

#[derive(Debug)]
pub struct Leintes {
    pub entity: H_Entity,
    pub model: H_Model,
    pub amodel: H_ActiveModel,
    pub column: H_Column,
}

#[derive(Debug)]
pub struct Szamla {
    pub entity: SB_Entity,
    pub model: SB_Model,
    pub amodel: SB_ActiveModel,
    pub column: SB_Column,
}

pub fn get_type_by_id(id: i32) -> Option<String> {
    match id {
        1 => Some(String::from("potlek")),
        2 => Some(String::from("leintes")),
        3 => Some(String::from("szamla")),
        _ => None,
    }
}

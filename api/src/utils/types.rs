pub enum Types {
    Potlek,
    Leintes,
    Szamla,
}

pub fn get_type_num(tip: Types) -> i32 {
    match tip {
        Types::Potlek => return 1,
        Types::Leintes => return 2,
        Types::Szamla => return 3,
    }
}

use strum_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter, Hash)]
pub enum Doors {
    Entrance,
    Throne,
    Toilet,
}

impl Doors {
    pub fn all_doors() -> Vec<Doors> {
        vec![Doors::Entrance, Doors::Throne, Doors::Toilet]
    }
}

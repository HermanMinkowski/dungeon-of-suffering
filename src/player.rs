use crate::equipment::Equipment;
#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub equipments: Equipment,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            equipments: Equipment::init_equipment(),
        }
    }
}

#[derive(Debug)]
pub struct Status {
    pub hungry: bool,
    pub hurt: bool,
    pub talked_to_mage: bool,
    pub signed_discharge: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            hungry: true,
            hurt: false,
            talked_to_mage: false,
            signed_discharge: false,
        }
    }
}

#[derive(Debug)]
pub struct Status {
    pub hungry: bool,
    pub hurt: bool,
    pub talked_to_mage: bool,
}

impl Status {
    pub fn new() -> Self {
        Status {
            hungry: false,
            hurt: false,
            talked_to_mage: false,
        }
    }
}

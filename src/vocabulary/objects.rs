use rust_i18n::t;

#[derive(Debug)]
pub struct Objects {
    pub bread: String,
    pub key: String,
}

impl Objects {
    pub fn new() -> Self {
        Self {
            bread: t!("objects.bread").to_string(),
            key: t!("objects.key").to_string(),
        }
    }

    pub fn refresh(&mut self) {
        self.bread = t!("objects.bread").to_string();
        self.key = t!("objects.key").to_string();
    }
}
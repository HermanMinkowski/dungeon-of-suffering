use rust_i18n::t;

#[derive(Debug)]
pub struct Objects {
    pub bread: String,
    pub key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Bread,
    Key,
    Unknown,
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

    pub fn parse(&self, input: &str) -> Object {
        if input == self.bread {
            Object::Bread
        } else if input == self.key {
            Object::Key
        } else {
            Object::Unknown
        }
    }
}

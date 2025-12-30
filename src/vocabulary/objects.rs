use rust_i18n::t;

#[derive(Debug)]
pub struct Objects {
    pub bread: String,
    pub notice: String,
    pub key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Bread,
    Notice,
    Key,
    Unknown,
}

impl Objects {
    pub fn new() -> Self {
        Self {
            bread: t!("object.bread").to_string(),
            notice: t!("object.notice").to_string(),
            key: t!("object.key").to_string(),
        }
    }

    pub fn refresh(&mut self) {
        self.bread = t!("object.bread").to_string();
        self.notice = t!("object.notice").to_string();
        self.key = t!("object.key").to_string();
    }

    pub fn parse(&self, input: &str) -> Object {
        if input == self.bread {
            Object::Bread
        } else if input == self.notice {
            Object::Notice
        } else if input == self.key {
            Object::Key
        } else {
            Object::Unknown
        }
    }
}

use rust_i18n::t;
use std::collections::HashMap;
use unicode_normalization::char::is_combining_mark;

#[derive(Debug)]
pub struct Objects {
    pub bread: String,
    pub notice: String,
    pub ginette: String,
    pub north: String,
    pub east: String,
    pub south: String,
    pub west: String,
    pub inn: String,
    pub key: String,

    lookup: HashMap<String, Object>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Bread,
    Notice,
    Ginette,
    North,
    East,
    South,
    West,
    Inn,
    Key,
    #[default]
    Unknown,
}

impl Objects {
    pub fn new() -> Self {
        let mut this = Self {
            bread: t!("object.bread").to_string(),
            notice: t!("object.notice").to_string(),
            ginette: t!("object.ginette").to_string(),
            north: t!("object.north").to_string(),
            east: t!("object.east").to_string(),
            south: t!("object.south").to_string(),
            west: t!("object.west").to_string(),
            inn: t!("object.inn").to_string(),
            key: t!("object.key").to_string(),
            lookup: HashMap::new(),
        };

        this.rebuild_lookup();
        this
    }

    pub fn refresh(&mut self) {
        self.bread = t!("object.bread").to_string();
        self.notice = t!("object.notice").to_string();
        self.ginette = t!("object.ginette").to_string();
        self.north = t!("object.north").to_string();
        self.east = t!("object.east").to_string();
        self.south = t!("object.south").to_string();
        self.west = t!("object.west").to_string();
        self.inn = t!("object.inn").to_string();
        self.key = t!("object.key").to_string();

        self.rebuild_lookup();
    }

    fn rebuild_lookup(&mut self) {
        self.lookup = HashMap::from([
            (normalize(&self.bread), Object::Bread),
            (normalize(&self.notice), Object::Notice),
            (normalize(&self.ginette), Object::Ginette),
            (normalize(&self.north), Object::North),
            (normalize(&self.east), Object::East),
            (normalize(&self.south), Object::South),
            (normalize(&self.west), Object::West),
            (normalize(&self.inn), Object::Inn),
            (normalize(&self.key), Object::Key),
        ]);
    }

    pub fn parse(&self, input: &str) -> Object {
        let key = normalize(input);
        self.lookup.get(&key).copied().unwrap_or(Object::Unknown)
    }
}

fn normalize(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;

    s.nfd()
        .filter(|c| !is_combining_mark(*c))
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

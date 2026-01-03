use rust_i18n::t;
use std::collections::HashMap;
use crate::cli::Cli;

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
    pub parchment: String,
    pub coal: String,

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
    Parchment,
    Coal,
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
            parchment: t!("object.parchment").to_string(),
            coal: t!("object.coal").to_string(),
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
        self.parchment = t!("object.parchment").to_string();
        self.coal = t!("object.coal").to_string();

        self.rebuild_lookup();
    }

    fn rebuild_lookup(&mut self) {
        self.lookup = HashMap::from([
            (Cli::normalize(&self.bread), Object::Bread),
            (Cli::normalize(&self.notice), Object::Notice),
            (Cli::normalize(&self.ginette), Object::Ginette),
            (Cli::normalize(&self.north), Object::North),
            (Cli::normalize(&self.east), Object::East),
            (Cli::normalize(&self.south), Object::South),
            (Cli::normalize(&self.west), Object::West),
            (Cli::normalize(&self.inn), Object::Inn),
            (Cli::normalize(&self.key), Object::Key),
            (Cli::normalize(&self.parchment), Object::Parchment),
            (Cli::normalize(&self.coal), Object::Coal),
        ]);
    }

    pub fn parse(&self, input: &str) -> Object {
        let key = Cli::normalize(input);
        self.lookup.get(&key).copied().unwrap_or(Object::Unknown)
    }
}

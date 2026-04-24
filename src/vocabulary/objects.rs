use crate::i18n::I18n;
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
    pub portcullis: String,
    pub water: String,
    pub chest: String,
    pub window: String,
    pub window2: String,
    pub bird: String,
    pub cookie: String,

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
    Portcullis,
    Water,
    Chest,
    Window,
    Window2,
    Bird,
    Cookie,
    #[default]
    Unknown,
}

impl Objects {
    pub fn new(i18n: &I18n) -> Self {
        let mut this = Self {
            bread: i18n.t("object-bread", None),
            notice: i18n.t("object-notice", None),
            ginette: i18n.t("object-ginette", None),
            north: i18n.t("object-north", None),
            east: i18n.t("object-east", None),
            south: i18n.t("object-south", None),
            west: i18n.t("object-west", None),
            inn: i18n.t("object-inn", None),
            key: i18n.t("object-key", None),
            parchment: i18n.t("object-parchment", None),
            coal: i18n.t("object-coal", None),
            portcullis: i18n.t("object-portcullis", None),
            water: i18n.t("object-water", None),
            chest: i18n.t("object-chest", None),
            window: i18n.t("object-window", None),
            window2: i18n.t("object-window2", None),
            bird: i18n.t("object-bird", None),
            cookie: i18n.t("object-cookie", None),
            lookup: HashMap::new(),
        };

        this.rebuild_lookup();
        this
    }

    pub fn refresh(&mut self, i18n: &I18n) {
        self.bread = i18n.t("object-bread", None);
        self.notice = i18n.t("object-notice", None);
        self.ginette = i18n.t("object-ginette", None);
        self.north = i18n.t("object-north", None);
        self.east = i18n.t("object-east", None);
        self.south = i18n.t("object-south", None);
        self.west = i18n.t("object-west", None);
        self.inn = i18n.t("object-inn", None);
        self.key = i18n.t("object-key", None);
        self.parchment = i18n.t("object-parchment", None);
        self.coal = i18n.t("object-coal", None);
        self.portcullis = i18n.t("object-portcullis", None);
        self.water = i18n.t("object-water", None);
        self.chest = i18n.t("object-chest", None);
        self.window = i18n.t("object-window", None);
        self.window2 = i18n.t("object-window2", None);
        self.bird = i18n.t("object-bird", None);
        self.cookie = i18n.t("object-cookie", None);

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
            (Cli::normalize(&self.portcullis), Object::Portcullis),
            (Cli::normalize(&self.water), Object::Water),
            (Cli::normalize(&self.chest), Object::Chest),
            (Cli::normalize(&self.window), Object::Window),
            (Cli::normalize(&self.window2), Object::Window2),
            (Cli::normalize(&self.bird), Object::Bird),
            (Cli::normalize(&self.cookie), Object::Cookie),
        ]);
    }

    pub fn parse(&self, input: &str) -> Object {
        let key = Cli::normalize(input);
        self.lookup.get(&key).copied().unwrap_or(Object::Unknown)
    }
}

use crate::i18n::I18n;
use crate::vocabulary::commands::Commands;
use crate::vocabulary::objects::Objects;
use crate::vocabulary::verbs::Verbs;

pub mod verbs;
pub mod objects;
pub mod commands;

#[derive(Debug)]
pub struct Vocabulary {
    pub verbs: Verbs,
    pub objects: Objects,
    pub commands: Commands,
}

impl Vocabulary {
    pub fn new(i18n: &I18n) -> Self {
        Self {
            verbs: Verbs::new(i18n),
            objects: Objects::new(i18n),
            commands: Commands::new(i18n),
        }
    }

    pub fn refresh(&mut self, i18n: &I18n) {
        self.verbs.refresh(i18n);
        self.objects.refresh(i18n);
        self.commands.refresh(i18n);
    }
}
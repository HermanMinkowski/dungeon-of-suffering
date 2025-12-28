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
    pub fn new() -> Self {
        Self {
            verbs: Verbs::new(),
            objects: Objects::new(),
            commands: Commands::new(),
        }
    }

    pub fn refresh(&mut self) {
        self.verbs.refresh();
        self.objects.refresh();
        self.commands.refresh();
    }
}
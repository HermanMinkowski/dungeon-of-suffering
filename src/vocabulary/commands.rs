use crate::i18n::I18n;

#[derive(Debug)]
pub struct Commands {
    pub help: String,
    pub equipment: String,
    pub quit: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Equipment,
    Quit,
    #[default]
    Unknown,
}

impl Commands {
    pub fn new(i18n: &I18n) -> Self {
        Self {
            help: i18n.t("command-help", None),
            equipment: i18n.t("command-equipment", None),
            quit: i18n.t("command-quit", None),
        }
    }

    pub fn refresh(&mut self, i18n: &I18n) {
        self.help = i18n.t("command-help", None);
        self.equipment = i18n.t("command-equipment", None);
        self.quit = i18n.t("command-quit", None);
    }

    pub fn parse(&self, input: &str) -> Command {
        if input == self.help {
            Command::Help
        } else if input == self.equipment {
            Command::Equipment
        } else if input == self.quit {
            Command::Quit
        } else {
            Command::Unknown
        }
    }
}

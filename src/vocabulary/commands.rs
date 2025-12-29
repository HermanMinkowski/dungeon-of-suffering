use rust_i18n::t;

#[derive(Debug)]
pub struct Commands {
    pub help: String,
    pub quit: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Quit,
    Unknown,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            help: t!("command.help").to_string(),
            quit: t!("command.quit").to_string(),
        }
    }

    pub fn refresh(&mut self) {
        self.help = t!("command.help").to_string();
        self.quit = t!("command.quit").to_string();
    }

    pub fn parse(&self, input: &str) -> Command {
        if input == self.help {
            Command::Help
        } else if input == self.quit {
            Command::Quit
        } else {
            Command::Unknown
        }
    }
}

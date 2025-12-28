use rust_i18n::t;

#[derive(Debug)]
pub struct Commands {
    pub help: String,
    pub quit: String,
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
}
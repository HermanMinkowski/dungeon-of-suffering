mod part1;
mod village;

use crate::doors::Doors;
use crate::equipment::Equipment;
use crate::player::Player;
use crate::state::State;
use crate::status::Status;
use crate::vocabulary::commands::Command;
use crate::vocabulary::commands::Commands;
use crate::vocabulary::objects::Objects;
use crate::vocabulary::verbs::Verbs;
use crate::vocabulary::Vocabulary;
use rust_i18n::{i18n, t};
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

i18n!("locales", fallback = "en");

#[derive(Debug)]
pub struct Game {
    pub player: Player,
    pub last_command: String,
    pub locked_doors: Vec<Doors>,
    pub status: Status,
    pub vocabulary: Vocabulary,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player: Player::default(),
            last_command: "".to_owned(),
            locked_doors: Doors::all_doors(),
            status: Status::new(),
            vocabulary: Vocabulary::new(),
        }
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.player.equipments = Equipment::init_equipment();
        self.last_command = "".to_owned();
        self.locked_doors = Doors::all_doors();
        self.status = Status::new();
    }

    pub fn start(&mut self) -> State<Game> {
        rust_i18n::set_locale("fr");
        self.vocabulary.refresh();
        println!("{}", t!("title"));

        State::with_input(Self::intro)
    }

    pub fn enter_name(&mut self) -> State<Game> {
        std::mem::swap(&mut self.player.name, &mut self.last_command);
        println!("{}", t!("messages.hello", name = self.player.name));
        State::no_input(Self::do_something)
    }

    pub fn end(&mut self) -> State<Game> {
        println!("{}", t!("message.end", name = self.player.name));
        State::completed(Self::end)
    }

    pub fn parse_command(&mut self) -> Vec<String> {
        self.last_command
            .nfd()
            .filter(|c| !is_combining_mark(*c))
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn verbs(&self) -> &Verbs {
        &self.vocabulary.verbs
    }

    pub fn objects(&self) -> &Objects {
        &self.vocabulary.objects
    }

    pub fn commands(&self) -> &Commands {
        &self.vocabulary.commands
    }

    pub fn handle_global_commands(&mut self, command: Command) -> bool {
        match command {
            Command::Help => {
                println!("{}", t!("help.text"));
                true
            }
            Command::Equipment => {
                self.player.equipments.list();
                true
            }
            Command::Quit => {
                println!("{}", t!("help.text"));
                true
            }
            _ => false,
        }
    }
}

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
use crate::parsed_input::ParsedInput;

i18n!("locales", fallback = "en");

#[derive(Debug)]
pub struct Game {
    pub player: Player,
    pub parsed_input: ParsedInput,
    pub locked_doors: Vec<Doors>,
    pub status: Status,
    pub vocabulary: Vocabulary,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            player: Player::default(),
            parsed_input: ParsedInput::default(),
            locked_doors: Doors::all_doors(),
            status: Status::new(),
            vocabulary: Vocabulary::new(),
        }
    }
}

impl Game {
    pub fn reset(&mut self) {
        self.player.equipments = Equipment::init_equipment();
        self.parsed_input = ParsedInput::default();
        self.locked_doors = Doors::all_doors();
        self.status = Status::new();
    }

    pub fn start(&mut self) -> State<Game> {
        let text_output = Some(t!("title").to_string());

        State::with_input(Self::intro, text_output)
    }

    pub fn enter_name(&mut self) -> State<Game> {
        //TODO
        //std::mem::swap(&mut self.player.name, &mut self.parsed_input.);
        let text_output = Some(t!("messages.hello", name = self.player.name).to_string());
        State::no_input(Self::do_something, text_output)
    }

    pub fn end(&mut self) -> State<Game> {
        println!("{}", t!("message.end", name = self.player.name));
        State::completed(Self::end)
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

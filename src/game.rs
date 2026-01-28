mod village;
mod cave_entrance;
mod cave_water;
use crate::doors::Doors;
use crate::equipment::Equipment;
use crate::parsed_input::ParsedInput;
use crate::player::Player;
use crate::state::{State, StateFn};
use crate::status::Status;
use crate::vocabulary::commands::Command;
use crate::vocabulary::commands::Commands;
use crate::vocabulary::objects::Objects;
use crate::vocabulary::verbs::Verbs;
use crate::vocabulary::Vocabulary;
use rust_i18n::t;

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

        self.display_text(Self::intro, text_output)
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

    pub fn handle_global_commands(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Help => self.text("help.text"),
            Command::Equipment => Some(self.player.equipments.list()),
            Command::Quit => self.text("help.text"),
            _ => None,
        }
    }

    fn default_answer(&mut self, next_state: StateFn<Game>) -> State<Game> {
        let text_output = self.text("cannot.do");
        self.display_text(next_state, text_output)
    }

    fn text(&self, key: &str) -> Option<String> {
        Some(t!(key).to_string())
    }

    fn text_with_object(&self, key: &str, object: &str) -> Option<String> {
        Some(t!(key, object = object).to_string())
    }

    fn display_text(
        &mut self,
        next_state: StateFn<Game>,
        text_output: Option<String>,
    ) -> State<Game> {
        State::with_input(next_state, text_output)
    }

    fn raw_object(&self) -> &str {
        &self.parsed_input.raw_object
    }
}

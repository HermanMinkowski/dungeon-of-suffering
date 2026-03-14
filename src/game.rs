mod village;
mod cave_entrance;
mod cave_water;
mod cave_mimic;

use crate::doors::Doors;
use crate::equipment::Equipment;
use crate::i18n::I18n;
use crate::parsed_input::ParsedInput;
use crate::player::Player;
use crate::state::{State, StateFn};
use crate::status::Status;
use crate::vocabulary::commands::Command;
use crate::vocabulary::Vocabulary;
use fluent_bundle::FluentArgs;

pub struct Game {
    pub player: Player,
    pub parsed_input: ParsedInput,
    pub locked_doors: Vec<Doors>,
    pub status: Status,
    pub vocabulary: Vocabulary,
    pub i18n: I18n,
}

impl Default for Game {
    fn default() -> Self {
        let ftl_source = include_str!("../locales/fr.ftl");
        let i18n = I18n::new("fr", ftl_source);

        Game {
            player: Player::default(),
            parsed_input: ParsedInput::default(),
            locked_doors: Doors::all_doors(),
            status: Status::new(),
            vocabulary: Vocabulary::new(&i18n),
            i18n,
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
        let text_output = Some(self.i18n.t("title", None));

        self.display_text(Self::intro, text_output)
    }

    pub fn end(&mut self) -> State<Game> {
        let mut args = FluentArgs::new();
        args.set("name", self.player.name.as_str());
        println!("{}", self.i18n.t("message-end", Some(&args)));
        State::completed(Self::end)
    }

    pub fn handle_global_commands(&mut self, command: Command) -> Option<String> {
        match command {
            Command::Help => self.text("help-text"),
            Command::Equipment => Some(self.player.equipments.list(&self.i18n)),
            Command::Quit => self.text("help-text"),
            _ => None,
        }
    }

    fn default_answer(&mut self, next_state: StateFn<Game>) -> State<Game> {
        let text_output = self.text("cannot-do");
        self.display_text(next_state, text_output)
    }

    fn text(&self, key: &str) -> Option<String> {
        Some(self.i18n.t(key, None))
    }

    fn text_with_object(&self, key: &str, object: &str) -> Option<String> {
        let mut args = FluentArgs::new();
        args.set("object", object);
        Some(self.i18n.t(key, Some(&args)))
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

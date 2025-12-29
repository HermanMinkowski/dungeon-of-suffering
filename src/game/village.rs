use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::commands::Command;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;
use rust_i18n::t;

impl Game {
    pub fn intro(&mut self) -> State<Game> {
        println!("{}", t!("intro.text"));

        State::with_input(Self::help)
    }

    pub fn help(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() != 1 {
            return State::with_input(Self::help);
        }

        let verb_part = parts.get(0).map(|s| s.as_str()).unwrap_or("");

        let command = self.vocabulary.commands.parse(verb_part);
        println!("{}", verb_part);

        match command {
            Command::Help => {
                println!("{}", t!("help.text"));
                State::with_input(Self::auberge)
            }
            _ => State::no_input(Self::help),
        }
    }

    pub fn auberge(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::auberge);
        }

        let verb_part = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let verb = self.vocabulary.verbs.parse(verb_part);
        let command = self.vocabulary.commands.parse(verb_part);

        let object_part = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let object = self.vocabulary.objects.parse(object_part);

        if self.handle_global_commands(command) {
            return State::with_input(Self::auberge);
        }

        //TODO complete logic

        match verb {
            Verb::Look => {
                println!("{}", t!("bravo"));
                State::no_input(Self::end)
            }
            Verb::Eat => match object {
                Object::Bread => {
                    println!("{}", t!("take.key"));
                    self.player.equipments.add(Item::new_default(ItemKind::Key));
                    State::with_input(Self::do_something)
                }
                _ => {
                    println!("{}", t!("cannot.take", object = object_part));
                    State::with_input(Self::do_something)
                }
            },
            _ => State::with_input(Self::do_something),
        }
    }
}

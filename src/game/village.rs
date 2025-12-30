use crate::equipment::ItemKind;
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
                State::with_input(Self::inn)
            }
            _ => State::no_input(Self::help),
        }
    }

    pub fn inn(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::inn);
        }

        let verb_part = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let verb = self.vocabulary.verbs.parse(verb_part);
        let command = self.vocabulary.commands.parse(verb_part);

        let object_part = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let object = self.vocabulary.objects.parse(object_part);

        if self.handle_global_commands(command) {
            return State::with_input(Self::inn);
        }

        //TODO complete logic

        match verb {
            Verb::Look =>  {
                    if object_part == "" {
                        println!("{}", "inn.look")
                    } else {
                        println!("{}", "look.nothing")
                    }
                    State::with_input(Self::inn)
                },
            Verb::Eat => match object {
                Object::Bread => {
                    if self.player.equipments.has(ItemKind::Bread.translation_key()) {
                        println!("{}", t!("inn.eat.bread"));
                        self.player.equipments.remove(ItemKind::Bread);
                        self.status.hungry = false;
                    } else {
                        println!("{}", t!("cannot.eat", object = object_part));
                    }
                    State::with_input(Self::inn)
                }
                _ => {
                    println!("{}", t!("cannot.eat", object = object_part));
                    State::with_input(Self::inn)
                }
            },
            Verb::Take => match object {
                Object::Notice => State::with_input(Self::inn),
                _ => State::with_input(Self::inn),
            },
            _ => State::with_input(Self::inn),
        }
    }
}

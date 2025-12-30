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
                        println!("{}", t!("inn.look"));
                    } else {
                        println!("{}", t!("look.nothing"));
                    }
                    State::with_input(Self::inn)
                },
            Verb::Eat => match object {
                Object::Bread => {
                    if self.player.equipments.has(ItemKind::Bread) {
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
                Object::Notice => {
                    if self.player.equipments.has(ItemKind::Notice) {
                        println!("{}", t!("inn.take.notice.has"));
                    } else {
                        println!("{}", t!("inn.take.notice"));
                        self.player.equipments.add(Item::new_default(ItemKind::Notice));
                    }

                    State::with_input(Self::inn)
                },
                _ => {
                    if object_part == "" {
                        println!("{}", t!("cannot.take.nothing"));
                    } else {
                        println!("{}", t!("cannot.take", object = object_part));
                    }

                    State::with_input(Self::inn)
                },
            },
            Verb::Talk => match object {
                Object::Ginette => {
                    println!("{}", t!("inn.talk.ginette"));
                    State::with_input(Self::inn)
                },
                _ => {
                    println!("{}", t!("cannot.talk"));
                    State::with_input(Self::inn)
                },
            },
            Verb::Go => match object {
                Object::East => {
                    if self.status.hungry {
                        println!("{}", t!("inn.go.east.hungry"));
                        return State::with_input(Self::inn)
                    }

                    if !self.player.equipments.has(ItemKind::Notice) {
                        println!("{}", t!("inn.go.east.notice"));
                        return State::with_input(Self::inn)
                    }

                    println!("{}", t!("inn.go.east"));
                    State::with_input(Self::do_something)
                },
                Object::Inn => {
                    println!("{}", t!("inn.go.inn"));
                    State::with_input(Self::inn)
                },
                _ => {
                    if object_part == "" {
                        println!("{}", t!("cannot.go.nowhere"));
                    } else {
                        println!("{}", t!("cannot.go", object = object_part));
                    }

                    State::with_input(Self::inn)
                },
            },
            _ => {
                println!("{}", t!("cannot.do"));
                State::with_input(Self::inn)
            },
        }
    }
}

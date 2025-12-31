use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::commands::Command;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;
use rust_i18n::t;

impl Game {
    pub fn intro(&mut self) -> State<Game> {
        let text_output = Some(t!("intro.text").to_string());

        State::with_input(Self::help, text_output)
    }

    pub fn help(&mut self) -> State<Game> {

        let command = self.parsed_input.command;

        if command == Command::Unknown {
            return State::with_input(Self::help, None);
        }

        match command {
            Command::Help => {
                let text_output = Some(t!("help.text").to_string());
                State::with_input(Self::inn, text_output)
            }
            _ => State::no_input(Self::help, None),
        }
    }

    pub fn inn(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        if self.handle_global_commands(command) {
            return State::with_input(Self::inn, None);
        }

        match verb {
            Verb::Look =>  self.inn_look(),
            Verb::Eat => self.inn_eat(),
            Verb::Take => self.inn_take(),
            Verb::Talk => self.inn_talk(),
            Verb::Go => self.inn_go(),
            _ => self.inn_default_answer(),
        }
    }

    fn inn_look(&mut self) -> State<Game> {
        let object_part = self.parsed_input.raw_object.to_string();
        let text_output: Option<String>;

        if object_part == "" {
            text_output = Some(t!("inn.look").to_string());
        } else {
            text_output = Some(t!("look.nothing").to_string());
        }
        self.display_text_inn(text_output)
    }

    fn inn_eat(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Bread => {
                let text_output: Option<String> ;

                if self.player.equipments.has(ItemKind::Bread) {
                    text_output = Some(t!("inn.eat.bread").to_string());
                    self.player.equipments.remove(ItemKind::Bread);
                    self.status.hungry = false;
                } else {
                    text_output = Some(t!("cannot.eat", object = self.raw_object()).to_string());
                }
                self.display_text_inn(text_output)
            }
            _ => {
                let text_output = Some(t!("cannot.eat", object = self.raw_object()).to_string());

                self.display_text_inn(text_output)
            }
        }
    }

    fn inn_take(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Notice => {
                let text_output: Option<String>;

                if self.player.equipments.has(ItemKind::Notice) {
                    text_output = Some(t!("inn.take.notice.has").to_string());
                } else {
                    text_output = Some(t!("inn.take.notice").to_string());
                    self.player.equipments.add(Item::new_default(ItemKind::Notice));
                }

                self.display_text_inn(text_output)                },
            _ => {
                let text_output: Option<String>;

                if self.raw_object() == "" {
                    text_output = Some(t!("cannot.take.nothing").to_string());
                } else {
                    text_output = Some(t!("cannot.take", object = self.raw_object()).to_string());
                }

                self.display_text_inn(text_output)
            },
        }
    }

    fn inn_talk(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Ginette => {
                let text_output = Some(t!("inn.talk.ginette").to_string());
                self.display_text_inn(text_output)
            },
            _ => {
                let text_output = Some(t!("cannot.talk").to_string());
                self.display_text_inn(text_output)
            },
        }
    }

    fn inn_go(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::East => {
                if self.status.hungry {
                    let text_output = Some(t!("inn.go.east.hungry").to_string());
                    return self.display_text_inn(text_output);
                }

                if !self.player.equipments.has(ItemKind::Notice) {
                    let text_output = Some(t!("inn.go.east.notice").to_string());
                    return self.display_text_inn(text_output);
                }

                let text_output = Some(t!("inn.go.east").to_string());
                State::with_input(Self::do_something, text_output)
            },
            Object::Inn => {
                let text_output = Some(t!("inn.go.inn").to_string());
                self.display_text_inn(text_output)
            },
            _ => {
                let text_output: Option<String> ;

                if self.raw_object() == "" {
                    text_output = Some(t!("cannot.go.nowhere").to_string());
                } else {
                    text_output = Some(t!("cannot.go", object = self.raw_object()).to_string());
                }

                self.display_text_inn(text_output)
            },
        }
    }

    fn inn_default_answer(&mut self) -> State<Game> {
        let text_output = Some(t!("cannot.do").to_string());
        self.display_text_inn(text_output)
    }

    fn display_text_inn(&mut self, text_output: Option<String>) -> State<Game> {
        State::with_input(Self::inn, text_output)
    }

    fn raw_object(&self) -> &String {
        &self.parsed_input.raw_object
    }
}

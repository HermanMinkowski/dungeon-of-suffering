use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::commands::Command;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;

impl Game {
    pub fn intro(&mut self) -> State<Game> {
        let text_output = self.text("intro.text");

        self.display_text(Self::help, text_output)
    }

    pub fn help(&mut self) -> State<Game> {

        let command = self.parsed_input.command;

        if command == Command::Unknown {
            return self.display_text(Self::help, None);
        }

        match command {
            Command::Help => {
                let text_output = self.text("help.text");
                State::with_input(Self::inn, text_output)
            }
            _ => State::no_input(Self::help, None),
        }
    }

    pub fn inn(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        let global_command_output = self.handle_global_commands(command);

        if global_command_output.is_some() {
            return self.display_text(Self::inn, global_command_output);
        }

        match verb {
            Verb::Look =>  self.inn_look(),
            Verb::Eat => self.inn_eat(),
            Verb::Take => self.inn_take(),
            Verb::Talk => self.inn_talk(),
            Verb::Go => self.inn_go(),
            _ => self.default_answer(Self::inn),
        }
    }

    fn inn_look(&mut self) -> State<Game> {
        let text_output: Option<String>;

        if self.raw_object().is_empty() {
            text_output = self.text("inn.look");
        } else {
            text_output = self.text("look.nothing");
        }
        self.display_text(Self::inn, text_output)
    }

    fn inn_eat(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Bread => {
                let text_output: Option<String> ;

                if self.player.equipments.has(ItemKind::Bread) {
                    text_output = self.text("inn.eat.bread");
                    self.player.equipments.remove(ItemKind::Bread);
                    self.status.hungry = false;
                } else {
                    text_output = self.text_with_object("cannot.eat", self.raw_object());
                }
                self.display_text(Self::inn, text_output)
            }
            _ => {
                let text_output = self.text_with_object("cannot.eat", self.raw_object());

                self.display_text(Self::inn, text_output)
            }
        }
    }

    fn inn_take(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Notice => {
                let text_output: Option<String>;

                if self.player.equipments.has(ItemKind::Notice) {
                    text_output = self.text("inn.take.notice.has");
                } else {
                    text_output = self.text("inn.take.notice");
                    self.player.equipments.add(Item::new_default(ItemKind::Notice));
                }

                self.display_text(Self::inn, text_output)             },
            _ => {
                let text_output: Option<String>;

                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.take.nothing");
                } else {
                    text_output = self.text_with_object("cannot.take", self.raw_object());
                }

                self.display_text(Self::inn, text_output)
            },
        }
    }

    fn inn_talk(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Ginette => {
                let text_output = self.text("inn.talk.ginette");
                self.display_text(Self::inn, text_output)
            },
            _ => {
                let text_output = self.text("cannot.talk");
                self.display_text(Self::inn, text_output)
            },
        }
    }

    fn inn_go(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::East => {
                if self.status.hungry {
                    let text_output = self.text("inn.go.east.hungry");
                    return self.display_text(Self::inn, text_output);
                }

                if !self.player.equipments.has(ItemKind::Notice) {
                    let text_output = self.text("inn.go.east.notice");
                    return self.display_text(Self::inn, text_output);
                }

                let text_output = self.text("inn.go.east");
                State::with_input(Self::do_something, text_output)
            },
            Object::Inn => {
                let text_output = self.text("inn.go.inn");
                self.display_text(Self::inn, text_output)
            },
            _ => {
                let text_output: Option<String> ;

                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.go.nowhere");
                } else {
                    text_output = self.text_with_object("cannot.go", self.raw_object())
                }

                self.display_text(Self::inn, text_output)
            },
        }
    }
}

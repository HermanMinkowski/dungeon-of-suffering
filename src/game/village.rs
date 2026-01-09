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
            Verb::Look => self.inn_look(),
            Verb::Take => self.inn_take(),
            Verb::Open => self.inn_open(),
            Verb::Use => self.inn_use(),
            Verb::Push => self.inn_push(),
            Verb::Go => self.inn_go(),
            Verb::Talk => self.inn_talk(),
            Verb::Jump => self.inn_jump(),
            Verb::Eat => self.inn_eat(),
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
        let text_output: Option<String>;

        match object {
            Object::Bread => {
                if self.player.equipments.has(ItemKind::Bread) {
                    text_output = self.text("inn.eat.bread");
                    self.player.equipments.remove(ItemKind::Bread);
                    self.status.hungry = false;
                } else {
                    text_output = self.text_with_object("cannot.eat", self.raw_object());
                }
            }
            _ => text_output = self.text("cannot.eat.nothing"),
        }

        self.display_text(Self::inn, text_output)
    }

    fn inn_take(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::Notice => {
                if self.player.equipments.has(ItemKind::Notice) {
                    text_output = self.text("inn.take.notice.has");
                } else {
                    text_output = self.text("inn.take.notice");
                    self.player
                        .equipments
                        .add(Item::new_default(ItemKind::Notice));
                }
            }
            _ => {
                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.take.nothing");
                } else {
                    text_output = self.text_with_object("cannot.take", self.raw_object());
                }
            }
        }

        self.display_text(Self::inn, text_output)
    }

    fn inn_talk(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::Ginette => text_output = self.text("inn.talk.ginette"),
            _ => text_output = self.text("cannot.talk"),
        }
        self.display_text(Self::inn, text_output)
    }

    fn inn_go(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::East => 'not_ready: {
                if self.status.hungry {
                    text_output = self.text("inn.go.east.hungry");
                    break 'not_ready;
                }

                if !self.player.equipments.has(ItemKind::Notice) {
                    text_output = self.text("inn.go.east.notice");
                    break 'not_ready;
                }

                text_output = self.text("inn.go.east");
                return State::with_input(Self::cave_entrance, text_output)
            }
            Object::Inn => text_output = self.text("inn.go.inn"),
            _ => {
                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.go.nowhere");
                } else {
                    text_output = self.text_with_object("cannot.go", self.raw_object())
                }
            }
        }

        self.display_text(Self::inn, text_output)
    }

    fn inn_open(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("cannot.do");
                self.display_text(Self::inn, text_output)
            }
        }
    }

    fn inn_use(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("cannot.do");
                self.display_text(Self::inn, text_output)
            }
        }
    }

    fn inn_push(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("cannot.do");
                self.display_text(Self::inn, text_output)
            }
        }
    }

    fn inn_jump(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("cannot.do");
                self.display_text(Self::inn, text_output)
            }
        }
    }
}

use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;

impl Game {
    pub fn cave_entrance(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        let global_command_output = self.handle_global_commands(command);

        if global_command_output.is_some() {
            return self.display_text(Self::cave_entrance, global_command_output);
        }

        match verb {
            Verb::Look =>  self.entrance_look(),
            Verb::Take => self.entrance_take(),
            Verb::Open => self.entrance_open(),
            Verb::Use => self.entrance_use(),
            Verb::Push => self.entrance_push(),
            Verb::Go => self.entrance_go(),
            Verb::Talk => self.entrance_talk(),
            Verb::Jump => self.entrance_jump(),
            Verb::Eat => self.entrance_eat(),
            _ => self.default_answer(Self::cave_entrance),
        }
    }

    fn entrance_look(&mut self) -> State<Game> {
        let text_output: Option<String>;

        if self.raw_object().is_empty() {
            text_output = self.text("entrance.look");
        } else {
            return self.entrance_look_object()
        }
        self.display_text(Self::cave_entrance, text_output)
    }

    fn entrance_look_object(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::Parchment => text_output = self.text("entrance.look.parchment"),
            Object::Coal => text_output = self.text("entrance.look.coal"),
            _ => text_output = self.text("look.nothing")
        }

        self.display_text(Self::cave_entrance, text_output)
    }

    fn entrance_eat(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Bread => {
                let text_output: Option<String> ;

                if self.player.equipments.has(ItemKind::Bread) {
                    text_output = self.text("TODO");
                    self.player.equipments.remove(ItemKind::Bread);
                    self.status.hungry = false;
                } else {
                    text_output = self.text_with_object("TODO", self.raw_object());
                }
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = self.text_with_object("TODO", self.raw_object());

                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn entrance_take(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Notice => {
                let text_output: Option<String>;

                if self.player.equipments.has(ItemKind::Notice) {
                    text_output = self.text("TODO");
                } else {
                    text_output = self.text("TODO");
                    self.player.equipments.add(Item::new_default(ItemKind::Notice));
                }

                self.display_text(Self::cave_entrance, text_output)             },
            _ => {
                let text_output: Option<String>;

                if self.raw_object().is_empty() {
                    text_output = self.text("TODO");
                } else {
                    text_output = self.text_with_object("TODO", self.raw_object());
                }

                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_talk(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::Ginette => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
            _ => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_go(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            Object::East => {
                if self.status.hungry {
                    let text_output = self.text("TODO");
                    return self.display_text(Self::cave_entrance, text_output);
                }

                if !self.player.equipments.has(ItemKind::Notice) {
                    let text_output = self.text("TODO");
                    return self.display_text(Self::cave_entrance, text_output);
                }

                let text_output = self.text("TODO");
                State::with_input(Self::do_something, text_output)
            },
            Object::Inn => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
            _ => {
                let text_output: Option<String> ;

                if self.raw_object().is_empty() {
                    text_output = self.text("TODO");
                } else {
                    text_output = self.text_with_object("TODO", self.raw_object())
                }

                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_open(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_use(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_push(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }

    fn entrance_jump(&mut self) -> State<Game> {
        let object = self.parsed_input.object;

        match object {
            _ => {
                let text_output = self.text("TODO");
                self.display_text(Self::cave_entrance, text_output)
            },
        }
    }
}

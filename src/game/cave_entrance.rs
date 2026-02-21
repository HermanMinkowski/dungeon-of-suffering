use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;

impl Game {
    pub fn cave_entrance(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        if let Some(output) = self.handle_global_commands(command) {
            return self.display_text(Self::cave_entrance, Some(output));
        }

        match verb {
            Verb::Look => self.entrance_look(),
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
        if !self.raw_object().is_empty() {
            return self.entrance_look_object();
        }

        let text_key = if self.status.signed_discharge {
            "entrance.look.signed"
        } else {
            "entrance.look"
        };

        self.display_text(Self::cave_entrance, self.text(text_key))
    }

    fn entrance_look_object(&mut self) -> State<Game> {
        let text_key = match self.parsed_input.object {
            Object::Parchment => "entrance.look.parchment",
            Object::Coal => "entrance.look.coal",
            _ => "look.nothing",
        };

        self.display_text(Self::cave_entrance, self.text(text_key))
    }

    fn entrance_eat(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("cannot.eat.nothing")
        } else {
            self.text_with_object("cannot.eat", self.raw_object())
        };

        self.display_text(Self::cave_entrance, text_output)
    }

    fn entrance_take(&mut self) -> State<Game> {
        match self.parsed_input.object {
            Object::Coal => {
                let text_output = if self.player.equipments.has(ItemKind::Coal) {
                    self.text("entrance.take.coal.again")
                } else {
                    self.player
                        .equipments
                        .add(Item::new_default(ItemKind::Coal));
                    self.text("entrance.take.coal")
                };

                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = if self.raw_object().is_empty() {
                    self.text("cannot.take.nothing")
                } else {
                    self.text_with_object("cannot.take", self.raw_object())
                };

                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn entrance_talk(&mut self) -> State<Game> {
        self.display_text(Self::inn, self.text("cannot.talk"))
    }

    fn entrance_go(&mut self) -> State<Game> {
        if self.go_and_has_not_signed_discharge(self.parsed_input.object) {
            let text_output = self.text("entrance.go.no.discharge");
            return self.display_text(Self::cave_entrance, text_output);
        }

        match self.parsed_input.object {
            Object::South => {
                let text_output = self.text("entrance.go.south");
                State::with_input(Self::cave_water, text_output)
            }
            Object::East => {
                let text_output = self.text("entrance.go.east");
                self.display_text(Self::cave_entrance, text_output)
            }
            Object::North => {
                let text_output = self.text("entrance.go.north");
                self.display_text(Self::cave_entrance, text_output)
            }
            Object::West => {
                let text_output = self.text("entrance.go.west");
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = if self.raw_object().is_empty() {
                    self.text("cannot.go.nowhere")
                } else {
                    self.text_with_object("cannot.go", self.raw_object())
                };

                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn go_and_has_not_signed_discharge(&mut self, object: Object) -> bool {
        !self.status.signed_discharge
            && (object == Object::East || object == Object::North || object == Object::South)
    }

    fn entrance_open(&mut self) -> State<Game> {
        match self.parsed_input.object {
            Object::Portcullis => {
                let text_output = self.text("entrance.open.portcullis");
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = if self.raw_object().is_empty() {
                    self.text("cannot.open.nothing")
                } else {
                    self.text_with_object("cannot.open", self.raw_object())
                };

                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn entrance_use(&mut self) -> State<Game> {
        match self.parsed_input.object {
            Object::Coal => {
                let text_output = if self.status.signed_discharge {
                    self.text("entrance.use.coal.again")
                } else {
                    self.status.signed_discharge = true;
                    self.text("entrance.use.coal")
                };

                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = self.text("cannot.use");
                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn entrance_push(&mut self) -> State<Game> {
        match self.parsed_input.object {
            Object::Portcullis => {
                let text_output = self.text("entrance.push.portcullis");
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = self.text("cannot.push");
                self.display_text(Self::cave_entrance, text_output)
            }
        }
    }

    fn entrance_jump(&mut self) -> State<Game> {
        let text_output = self.text("cannot.jump");
        self.display_text(Self::cave_entrance, text_output)
    }
}

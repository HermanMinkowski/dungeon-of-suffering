use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;

impl Game {
    pub fn cave_water(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        let global_command_output = self.handle_global_commands(command);

        if global_command_output.is_some() {
            return self.display_text(Self::cave_water, global_command_output);
        }

        match verb {
            Verb::Look => self.water_look(),
            Verb::Take => self.water_take(),/*
            Verb::Open => self.water_open(),
            Verb::Use => self.water_use(),
            Verb::Push => self.water_push(),
            Verb::Go => self.water_go(),
            Verb::Talk => self.water_talk(),
            Verb::Jump => self.water_jump(),
            Verb::Eat => self.water_eat(),*/
            _ => self.default_answer(Self::cave_water),
        }
    }

    fn water_look(&mut self) -> State<Game> {
        let text_output: Option<String>;

        if self.raw_object().is_empty() {
            text_output = self.text("water.look");
        } else {
            return self.water_look_object();
        }
        self.display_text(Self::cave_water, text_output)
    }

    fn water_look_object(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::Water => text_output = self.text("water.look.water"),
            _ => text_output = self.text("look.nothing"),
        }

        self.display_text(Self::cave_water, text_output)
    }

    fn water_take(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::Water => {
                text_output = self.text("water.take.water");
                self.display_text(Self::cave_water, text_output)
            }
            _ => {
                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.take.nothing");
                } else {
                    text_output = self.text_with_object("cannot.take", self.raw_object());
                }

                self.display_text(Self::cave_water, text_output)
            }
        }
    }


    /*
        fn entrance_eat(&mut self) -> State<Game> {
            let text_output: Option<String>;

            if self.raw_object().is_empty() {
                text_output = self.text("cannot.eat.nothing");
            } else {
                text_output = self.text_with_object("cannot.eat", self.raw_object());
            }

            self.display_text(Self::cave_entrance, text_output)
        }

        fn entrance_take(&mut self) -> State<Game> {
            let object = self.parsed_input.object;
            let text_output: Option<String>;

            match object {
                Object::Coal => {
                    if self.player.equipments.has(ItemKind::Coal) {
                        text_output = self.text("entrance.take.coal.again");
                    } else {
                        text_output = self.text("entrance.take.coal");
                        self.player
                            .equipments
                            .add(Item::new_default(ItemKind::Coal));
                    }

                    self.display_text(Self::cave_entrance, text_output)
                }
                _ => {
                    if self.raw_object().is_empty() {
                        text_output = self.text("cannot.take.nothing");
                    } else {
                        text_output = self.text_with_object("cannot.take", self.raw_object());
                    }

                    self.display_text(Self::cave_entrance, text_output)
                }
            }
        }

        fn entrance_talk(&mut self) -> State<Game> {
            self.display_text(Self::inn, self.text("cannot.talk"))
        }

        fn entrance_go(&mut self) -> State<Game> {
            let object = self.parsed_input.object;
            let text_output: Option<String>;

            if self.go_and_has_not_signed_dischage(object) {
                text_output = self.text("entrance.go.no.discharge");
                return self.display_text(Self::cave_entrance, text_output);
            }

            match object {
                Object::South => {
                    text_output = self.text("entrance.go.south");
                    State::with_input(Self::cave_entrance, text_output)
                }
                Object::East => {
                    text_output = self.text("entrance.go.east");
                    self.display_text(Self::cave_entrance, text_output)
                }
                Object::North => {
                    text_output = self.text("entrance.go.north");
                    self.display_text(Self::cave_entrance, text_output)
                }
                Object::West => {
                    text_output = self.text("entrance.go.west");
                    self.display_text(Self::cave_entrance, text_output)
                }
                _ => {
                    if self.raw_object().is_empty() {
                        text_output = self.text("cannot.go.nowhere");
                    } else {
                        text_output = self.text_with_object("cannot.go", self.raw_object())
                    }

                    self.display_text(Self::cave_entrance, text_output)
                }
            }
        }

        fn go_and_has_not_signed_dischage(&mut self, object: Object) -> bool {
            !self.status.signed_discharge
                && (object == Object::East || object == Object::North || object == Object::South)
        }

        fn entrance_open(&mut self) -> State<Game> {
            let object = self.parsed_input.object;
            let text_output: Option<String>;

            match object {
                Object::Portcullis => {
                    text_output = self.text("entrance.open.portcullis");
                    self.display_text(Self::cave_entrance, text_output)
                }
                _ => {
                    if self.raw_object().is_empty() {
                        text_output = self.text("cannot.open.nothing");
                    } else {
                        text_output = self.text_with_object("cannot.open", self.raw_object())
                    }

                    self.display_text(Self::cave_entrance, text_output)
                }
            }
        }

        fn entrance_use(&mut self) -> State<Game> {
            let object = self.parsed_input.object;
            let text_output: Option<String>;

            match object {
                Object::Coal => {
                    if self.status.signed_discharge {
                        text_output = self.text("entrance.use.coal.again");
                    } else {
                        text_output = self.text("entrance.use.coal");
                        self.status.signed_discharge = true;
                    }

                    self.display_text(Self::cave_entrance, text_output)
                }
                _ => {
                    text_output = self.text("cannot.use");
                    self.display_text(Self::cave_entrance, text_output)
                }
            }
        }

        fn entrance_push(&mut self) -> State<Game> {
            let object = self.parsed_input.object;
            let text_output: Option<String>;

            match object {
                Object::Portcullis => {
                    text_output = self.text("entrance.push.portcullis");
                    self.display_text(Self::cave_entrance, text_output)
                }
                _ => {
                    text_output = self.text("cannot.push");
                    self.display_text(Self::cave_entrance, text_output)
                }
            }
        }

        fn entrance_jump(&mut self) -> State<Game> {
            let text_output = self.text("cannot.jump");
            self.display_text(Self::cave_entrance, text_output)
        }*/
}

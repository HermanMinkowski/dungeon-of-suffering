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
            Verb::Take => self.water_take(),
            Verb::Go => self.water_go(),
            Verb::Talk => self.water_talk(),
            Verb::Eat => self.water_eat(),

            /*
            Verb::Open => self.water_open(),
            Verb::Use => self.water_use(),
            Verb::Push => self.water_push(),
            Verb::Jump => self.water_jump(),
            */
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

    fn water_eat(&mut self) -> State<Game> {
        let text_output: Option<String>;

        if self.raw_object().is_empty() {
            text_output = self.text("cannot.eat.nothing");
        } else {
            text_output = self.text_with_object("cannot.eat", self.raw_object());
        }

        self.display_text(Self::cave_water, text_output)
    }

    fn water_talk(&mut self) -> State<Game> {
        self.display_text(Self::inn, self.text("cannot.talk"))
    }

    fn water_go(&mut self) -> State<Game> {
        let object = self.parsed_input.object;
        let text_output: Option<String>;

        match object {
            Object::North => {
                text_output = self.text("water.go.north");
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                if self.raw_object().is_empty() {
                    text_output = self.text("cannot.go.nowhere");
                } else {
                    text_output = self.text_with_object("cannot.go", self.raw_object())
                }

                self.display_text(Self::cave_water, text_output)
            }
        }
    }

    /*


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

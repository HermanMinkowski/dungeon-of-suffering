use crate::state::State;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;
use crate::Game;

impl Game {
    pub fn cave_mimic(&mut self) -> State<Game> {
        let command = self.parsed_input.command;
        let verb = self.parsed_input.verb;

        let global_command_output = self.handle_global_commands(command);

        if global_command_output.is_some() {
            return self.display_text(Self::cave_water, global_command_output);
        }

        match verb {
            Verb::Look => self.mimic_look(),
            /*Verb::Take => self.water_take(),
            Verb::Go => self.water_go(),
            Verb::Talk => self.water_talk(),
            Verb::Eat => self.water_eat(),
            Verb::Open => self.water_open(),
            Verb::Use => self.water_use(),
            Verb::Push => self.water_push(),
            Verb::Jump => self.water_jump(),*/

            _ => self.default_answer(Self::cave_mimic),
        }
    }

    
    fn mimic_look(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("mimic.look")
        } else {
            return self.mimic_look_object();
        };

        self.display_text(Self::cave_water, text_output)
    }

    fn mimic_look_object(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Water => {
                self.status.saw_the_light = true;
                self.text("water.look.water")
            }
            _ => self.text("look.nothing"),
        };

        self.display_text(Self::cave_water, text_output)
    }
/*
    fn water_take(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Water => self.text("water.take.water"),
            _ => {
                if self.raw_object().is_empty() {
                    self.text("cannot.take.nothing")
                } else {
                    return State::with_input(
                        Self::cave_entrance,
                        self.text_with_object("cannot.take", self.raw_object()),
                    );
                }
            }
        };

        self.display_text(Self::cave_water, text_output)
    }

    fn water_eat(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("cannot.eat.nothing")
        } else {
            self.text_with_object("cannot.eat", self.raw_object())
        };

        self.display_text(Self::cave_water, text_output)
    }

    fn water_talk(&mut self) -> State<Game> {
        self.display_text(Self::inn, self.text("cannot.talk"))
    }

    fn water_go(&mut self) -> State<Game> {
        match self.parsed_input.object {
            Object::North => {
                let text_output = self.text("water.go.north");
                self.display_text(Self::cave_entrance, text_output)
            }
            _ => {
                let text_output = if self.raw_object().is_empty() {
                    self.text("cannot.go.nowhere")
                } else {
                    self.text_with_object("cannot.go", self.raw_object())
                };

                self.display_text(Self::cave_water, text_output)
            }
        }
    }

    fn water_open(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("cannot.open.nothing")
        } else {
            self.text_with_object("cannot.open", self.raw_object())
        };

        self.display_text(Self::cave_entrance, text_output)
    }

    fn water_use(&mut self) -> State<Game> {
        let text_output = self.text("cannot.use");
        self.display_text(Self::cave_water, text_output)
    }

    fn water_push(&mut self) -> State<Game> {
        let text_output = self.text("cannot.push");
        self.display_text(Self::cave_water, text_output)
    }

    //TODO
    fn water_jump(&mut self) -> State<Game> {
        if self.parsed_input.object == Object::Water {
            return self.display_text(Self::cave_entrance, self.text("water.jump.water"));
        }

        self.display_text(Self::cave_water, self.text("cannot.jump"))
    }
    
 */
}

use crate::equipment::ItemKind;
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
            Verb::Take => self.mimic_take(),
            Verb::Go => self.mimic_go(),
            Verb::Talk => self.mimic_talk(),
            Verb::Eat => self.mimic_eat(),
            Verb::Open => self.mimic_open(), /*
            Verb::Use => self.water_use(),
            Verb::Push => self.water_push(),
            Verb::Jump => self.water_jump(),*/
            _ => self.default_answer(Self::cave_mimic),
        }
    }

    fn mimic_look(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("mimic-look")
        } else {
            return self.mimic_look_object();
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_look_object(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Water => self.text("mimic-look-water"),
            Object::Chest => self.text("mimic-look-chest"),
            Object::Window => self.text("mimic-look-window"),
            Object::Window2 => self.text("mimic-look-window"),
            Object::Bird => self.text("mimic-look-bird"),
            _ => self.text("look-nothing"),
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_take(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Chest => self.text("mimic-take-chest"),
            Object::Window => self.text("mimic-take-window"),
            Object::Window2 => self.text("mimic-take-window"),
            Object::Bird => self.text("mimic-take-bird"),
            Object::Key => self.text("mimic-take-key"),
            Object::Water => self.text("mimic-take-water"),
            _ => {
                if self.raw_object().is_empty() {
                    self.text("cannot-take-nothing")
                } else {
                    return State::with_input(
                        Self::cave_mimic,
                        self.text_with_object("cannot-take", self.raw_object()),
                    );
                }
            }
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_go(&mut self) -> State<Game> {
        let text_output = if self.raw_object().is_empty() {
            self.text("cannot-go-nowhere")
        } else {
            self.text_with_object("cannot-go", self.raw_object())
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_talk(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Bird => self.text("mimic-talk-bird"),
            _ => {
                if self.raw_object().is_empty() {
                    self.text("cannot-talk-yourself")
                } else {
                    return State::with_input(
                        Self::cave_mimic,
                        self.text_with_object("cannot-talk", self.raw_object()),
                    );
                }
            }
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_eat(&mut self) -> State<Game> {
        let text_output = match self.parsed_input.object {
            Object::Cookie => {
                if self.player.equipments.has(ItemKind::Cookie) {
                    self.text("mimic-eat-cookie")
                } else {
                    self.text_with_object("cannot-eat", self.raw_object())
                }
            }
            _ => {
                if self.raw_object().is_empty() {
                    self.text("cannot-eat-nothing")
                } else {
                    return State::with_input(
                        Self::cave_mimic,
                        self.text_with_object("cannot-eat", self.raw_object()),
                    );
                }
            }
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    fn mimic_open(&mut self) -> State<Game> {
        //TODO IMPLEMENT PROPER DEATH
        if self.parsed_input.object == Object::Chest {
            return self.display_text(Self::intro, self.text("mimic-open-chest-dead"));
        }

        let text_output = if self.raw_object().is_empty() {
            self.text("cannot-open-nothing")
        } else {
            self.text_with_object("cannot-open", self.raw_object())
        };

        self.display_text(Self::cave_mimic, text_output)
    }

    //TODO IMPLEMENT check if looked through window. if yes use cookie. Should have check through window before tanking to bird also.

    fn water2_use(&mut self) -> State<Game> {
        let text_output = self.text("cannot-use");
        self.display_text(Self::cave_water, text_output)
    }

    fn water2_push(&mut self) -> State<Game> {
        let text_output = self.text("cannot-push");
        self.display_text(Self::cave_water, text_output)
    }

    fn water2_jump(&mut self) -> State<Game> {
        if self.parsed_input.object == Object::Water {
            return self.display_text(Self::cave_entrance, self.text("water-jump-water"));
        }

        self.display_text(Self::cave_water, self.text("cannot-jump"))
    }
}

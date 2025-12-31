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
        let parts = self.parse_command();

        if parts.len() != 1 {
            return State::with_input(Self::help, None);
        }

        let verb_part = parts.get(0).map(|s| s.as_str()).unwrap_or("");

        let command = self.vocabulary.commands.parse(verb_part);
        println!("{}", verb_part);

        match command {
            Command::Help => {
                let text_output = Some(t!("help.text").to_string());
                State::with_input(Self::inn, text_output)
            }
            _ => State::no_input(Self::help, None),
        }
    }

    pub fn inn(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::inn, None);
        }

        let verb_part = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let verb = self.vocabulary.verbs.parse(verb_part);
        let command = self.vocabulary.commands.parse(verb_part);

        let object_part = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let object = self.vocabulary.objects.parse(object_part);

        if self.handle_global_commands(command) {
            return State::with_input(Self::inn, None);
        }

        match verb {
            Verb::Look =>  {
                    let text_output: Option<String>;

                    if object_part == "" {
                        text_output = Some(t!("inn.look").to_string());
                    } else {
                        text_output = Some(t!("look.nothing").to_string());
                    }
                    State::with_input(Self::inn, text_output)
                },
            Verb::Eat => match object {
                Object::Bread => {
                    let text_output: Option<String> ;

                    if self.player.equipments.has(ItemKind::Bread) {
                        text_output = Some(t!("inn.eat.bread").to_string());
                        self.player.equipments.remove(ItemKind::Bread);
                        self.status.hungry = false;
                    } else {
                        text_output = Some(t!("cannot.eat", object = object_part).to_string());
                    }
                    State::with_input(Self::inn, text_output)
                }
                _ => {
                    let text_output = Some(t!("cannot.eat", object = object_part).to_string());
                    State::with_input(Self::inn, text_output)
                }
            },
            Verb::Take => match object {
                Object::Notice => {
                    let text_output: Option<String>;

                    if self.player.equipments.has(ItemKind::Notice) {
                        text_output = Some(t!("inn.take.notice.has").to_string());
                    } else {
                        text_output = Some(t!("inn.take.notice").to_string());
                        self.player.equipments.add(Item::new_default(ItemKind::Notice));
                    }

                    State::with_input(Self::inn, text_output)
                },
                _ => {
                    let text_output: Option<String>;

                    if object_part == "" {
                        text_output = Some(t!("cannot.take.nothing").to_string());
                    } else {
                        text_output = Some(t!("cannot.take", object = object_part).to_string());
                    }

                    State::with_input(Self::inn, text_output)
                },
            },
            Verb::Talk => match object {
                Object::Ginette => {
                    let text_output = Some(t!("inn.talk.ginette").to_string());
                    State::with_input(Self::inn, text_output)
                },
                _ => {
                    let text_output = Some(t!("cannot.talk").to_string());
                    State::with_input(Self::inn, text_output)
                },
            },
            Verb::Go => match object {
                Object::East => {
                    if self.status.hungry {
                        let text_output = Some(t!("inn.go.east.hungry").to_string());
                        return State::with_input(Self::inn, text_output);
                    }

                    if !self.player.equipments.has(ItemKind::Notice) {
                        let text_output = Some(t!("inn.go.east.notice").to_string());
                        return State::with_input(Self::inn, text_output);
                    }

                    let text_output = Some(t!("inn.go.east").to_string());
                    State::with_input(Self::do_something, text_output)
                },
                Object::Inn => {
                    let text_output = Some(t!("inn.go.inn").to_string());
                    State::with_input(Self::inn, text_output)
                },
                _ => {
                    let text_output: Option<String> ;

                    if object_part == "" {
                        text_output = Some(t!("cannot.go.nowhere").to_string());
                    } else {
                        text_output = Some(t!("cannot.go", object = object_part).to_string());
                    }

                    State::with_input(Self::inn, text_output)
                },
            },
            _ => {
                let text_output = Some(t!("cannot.do").to_string());
                State::with_input(Self::inn, text_output)
            },
        }
    }
}

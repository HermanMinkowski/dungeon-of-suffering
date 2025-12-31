use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::Game;
use rust_i18n::t;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;

impl Game {
    pub fn do_something(&mut self) -> State<Game> {
        let verb = self.parsed_input.verb;
        let object_part = self.parsed_input.raw_object.to_string();
        let object = self.parsed_input.object;

        if verb == Verb::Unknown {
            return State::with_input(Self::do_something, None);
        }

        match verb {
            Verb::Unknown => {
                let text_output = Some(t!("do").to_string());
                State::with_input(Self::do_something, text_output)
            }
            Verb::Go => {
                if self.player.equipments.has(ItemKind::Key) {
                    let text_output = Some(t!("bravo").to_string());
                    State::no_input(Self::end, text_output)
                } else {
                    State::with_input(Self::do_something, None)
                }
            }
            Verb::Take => match object {
                Object::Key => {
                    let text_output = Some(t!("take.key").to_string());
                    self.player.equipments.add(Item::new_default(ItemKind::Key));
                    State::with_input(Self::do_something, text_output)
                }
                _ => {
                    let text_output = Some(t!("cannot.take", object = object_part).to_string());
                    State::with_input(Self::do_something, text_output)
                }
            },
            _ => State::with_input(Self::do_something, None),
        }
    }
}

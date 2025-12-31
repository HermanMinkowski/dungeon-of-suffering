use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::Game;
use rust_i18n::t;

impl Game {
    pub fn do_something(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::do_something, None);
        }

        let verb = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let object = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let take = t!("verb.take");
        let key = t!("object.key");

        match verb {
            "" => {
                let text_output = Some(t!("do").to_string());
                State::with_input(Self::do_something, text_output)
            }
            "exit" => {
                if self.player.equipments.has(ItemKind::Key) {
                    let text_output = Some(t!("bravo").to_string());
                    State::no_input(Self::end, text_output)
                } else {
                    State::with_input(Self::do_something, None)
                }
            }
            _ if verb == take => match object {
                _ if object == key => {
                    let text_output = Some(t!("take.key").to_string());
                    self.player.equipments.add(Item::new_default(ItemKind::Key));
                    State::with_input(Self::do_something, text_output)
                }
                _ => {
                    let text_output = Some(t!("cannot.take", object = object).to_string());
                    State::with_input(Self::do_something, text_output)
                }
            },
            _ => State::with_input(Self::do_something, None),
        }
    }
}

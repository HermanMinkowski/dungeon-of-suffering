use crate::equipment::{Item, ItemKind};
use crate::state::State;
use crate::Game;
use rust_i18n::t;

impl Game {
    pub fn do_something(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::do_something);
        }

        let verb = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let object = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let take = t!("verb.take");
        let key = t!("object.key");

        match verb {
            "" => {
                println!("{}", t!("do"));
                State::with_input(Self::do_something)
            }
            "exit" => {
                if self.player.equipments.has(ItemKind::Key) {
                    println!("{}", t!("bravo"));
                    State::no_input(Self::end)
                } else {
                    State::with_input(Self::do_something)
                }
            }
            _ if verb == take => match object {
                _ if object == key => {
                    println!("{}", t!("take.key"));
                    self.player.equipments.add(Item::new_default(ItemKind::Key));
                    State::with_input(Self::do_something)
                }
                _ => {
                    println!("{}", t!("cannot.take", object = object));
                    State::with_input(Self::do_something)
                }
            },
            _ => State::with_input(Self::do_something),
        }
    }
}

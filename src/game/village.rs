use crate::state::State;
use crate::Game;
use rust_i18n::t;
use crate::equipment::{Item, ItemKind};

impl Game {
    pub fn intro(&mut self) -> State<Game> {
        println!("{}", t!("intro.text"));

        State::with_input(Self::help)
    }

    pub fn help(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() != 1 {
            return State::with_input(Self::help);
        }

        let verb = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let help = t!("help");

        match verb {
            "" => {
                println!("{}", t!("do"));
                State::no_input(Self::help)
            }
            _ if verb == help => {
                println!("{}", t!("help.text"));
                State::with_input(Self::do_something)
            }
            _ => State::no_input(Self::help),
        }
    }

    pub fn auberge(&mut self) -> State<Game> {
        let parts = self.parse_command();

        if parts.len() < 1 || parts.len() > 2 {
            return State::with_input(Self::do_something);
        }

        //TODO put verbs and object in a struct reset at language change

        let verb = parts.get(0).map(|s| s.as_str()).unwrap_or("");
        let object = parts.get(1).map(|s| s.as_str()).unwrap_or("");
        let look = t!("verb.look");
        let eat = t!("verb.eat");
        let take = t!("verb.take");
        let talk = t!("verb.talk");
        let go = t!("verb.go");
        let bread = t!("object.bread");

        //TODO complete logic

        match verb {
            "" => {
                println!("{}", t!("do"));
                State::with_input(Self::do_something)
            }
            "exit" => {
                if self.player.equipments.has("key") {
                    println!("{}", t!("bravo"));
                    State::no_input(Self::end)
                } else {
                    State::with_input(Self::do_something)
                }
            }
            _ if verb == eat => match object {
                _ if object == bread => {
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

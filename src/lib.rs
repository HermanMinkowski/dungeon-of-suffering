use rust_i18n::i18n;

i18n!("locales", fallback = "en");

pub mod cli;
pub mod doors;
pub mod equipment;
pub mod game;
pub mod player;
pub mod state;
pub mod status;
pub mod vocabulary;

pub use cli::game_loop;
pub use game::Game;

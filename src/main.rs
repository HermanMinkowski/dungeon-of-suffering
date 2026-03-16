pub mod cli;
pub mod doors;
pub mod equipment;
pub mod game;
pub mod i18n;
pub mod player;
pub mod state;
pub mod status;
pub mod vocabulary;
pub mod ui;

pub mod native_ui;
pub mod parsed_input;

pub use game::Game;

fn main() {
    let ui = Box::new(native_ui::NativeUI);
    let mut cli = cli::Cli::new(ui);
    cli.game_loop();
}

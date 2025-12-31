extern crate rust_i18n;

use dungeon_of_suffering::cli;

fn main() {
    let mut cli = cli::Cli::default();
    cli.game_loop();}


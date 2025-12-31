use crate::state::State;
use crate::Game;

pub fn game_loop() {
    use std::io::Write;
    let mut game = Game::default();
    let mut state_function = State::no_input(Game::start, None);

    state_function = state_function(&mut game);

    while !state_function.completed {

        println!("{}", state_function.output);

        if state_function.requires_input {
            let mut buffer = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();
            game.last_command = buffer.trim().to_owned();
        } else {
            game.last_command = "".to_owned();
        }
        state_function = state_function(&mut game);
    }
}

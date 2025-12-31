use crate::parsed_input::ParsedInput;
use crate::state::State;
use crate::vocabulary::Vocabulary;
use crate::Game;
use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;
use std::io::Write;
pub struct Cli {
    game: Game,
    state_function: State<Game>,
    vocabulary: Vocabulary,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            game: Game::default(),
            state_function: State::no_input(Game::start, None),
            vocabulary: Vocabulary::new(),
        }
    }
}

impl Cli     {
    pub fn game_loop(&mut self) {
        rust_i18n::set_locale("fr");
        self.vocabulary.refresh();
        
        self.state_function = (self.state_function)(&mut self.game);

        while !self.state_function.completed {
            println!("{}", self.state_function.output);

            if self.state_function.requires_input {
                let mut buffer = String::new();
                print!("> ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut buffer).unwrap();
                self.game.parsed_input = self.parse_input(buffer.trim().to_owned());
            } else {
                self.game.parsed_input = ParsedInput::default();
            }
            self.state_function = (self.state_function)(&mut self.game);
        }
    }

    fn parse_input(&mut self, input: String) -> ParsedInput {
        let normalized_input = self.normalize_input(input);
        self.generate_parsed_input(normalized_input)
    }

    fn normalize_input(&mut self, input: String) -> Vec<String> {
        input
            .nfd()
            .filter(|c| !is_combining_mark(*c))
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    fn generate_parsed_input(&mut self, normalize_input: Vec<String>) -> ParsedInput {
        if normalize_input.len() < 1 || normalize_input.len() > 2 {
            return ParsedInput::default();
        }

        let verb_part = normalize_input.get(0).map(|s| s.as_str()).unwrap_or("");
        let verb = self.vocabulary.verbs.parse(verb_part);
        let command = self.vocabulary.commands.parse(verb_part);

        let object_part = normalize_input.get(1).map(|s| s.as_str()).unwrap_or("");
        let object = self.vocabulary.objects.parse(object_part);

        ParsedInput::new(verb, command, object, object_part.to_string())
    }
}

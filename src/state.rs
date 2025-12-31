use std::ops::Deref;

pub type StateFn<T> = fn(&mut T) -> State<T>;
pub struct State<T> {
    pub function: StateFn<T>,
    pub requires_input: bool,
    pub completed: bool,
    pub output: String,
}

impl<T> State<T> {
    pub fn new(function: StateFn<T>, requires_input: bool, completed: bool, output: Option<String>) -> State<T> {
        State {
            function,
            requires_input,
            completed,
            output: output.unwrap_or_else(|| String::from("")),
        }
    }

    pub fn no_input(function: StateFn<T>, text_output: Option<String>) -> State<T> {
        State::new(function, false, false, text_output)
    }

    pub fn with_input(function: StateFn<T>, text_output: Option<String>) -> State<T> {
        State::new(function, true, false, text_output)
    }

    pub fn completed(function: StateFn<T>) -> State<T> {
        State::new(function, false, true, None)
    }
}

impl<T> Deref for State<T> {
    type Target = StateFn<T>;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

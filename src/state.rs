use std::ops::Deref;

pub type StateFn<T> = fn(&mut T) -> State<T>;
pub struct State<T> {
    pub function: StateFn<T>,
    pub requires_input: bool,
    pub completed: bool,
}

impl<T> State<T> {
    pub fn new(function: StateFn<T>, requires_input: bool, completed: bool) -> State<T> {
        State {
            function,
            requires_input,
            completed,
        }
    }

    pub fn no_input(function: StateFn<T>) -> State<T> {
        State::new(function, false, false)
    }

    pub fn with_input(function: StateFn<T>) -> State<T> {
        State::new(function, true, false)
    }

    pub fn completed(function: StateFn<T>) -> State<T> {
        State::new(function, false, true)
    }
}

impl<T> Deref for State<T> {
    type Target = StateFn<T>;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

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

use cli::Cli;
use ui::UI;
use wasm_bindgen::prelude::*;
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

thread_local! {
    static WASM_CLI: RefCell<Option<Cli>> = RefCell::new(None);
}

#[wasm_bindgen]
pub fn wasm_start() -> String {
    struct DummyUI;
    impl UI for DummyUI {
        fn print(&mut self, _s: &str) {}
        fn prompt(&mut self, _prompt: &str) -> String { String::new() }
    }

    WASM_CLI.with(|cell| {
        let mut maybe = cell.borrow_mut();
        *maybe = Some(Cli::new(Box::new(DummyUI)));
        maybe.as_mut().unwrap().start()
    })
}

#[wasm_bindgen]
pub fn wasm_submit(input: &str) -> String {
    WASM_CLI.with(|cell| {
        let mut maybe = cell.borrow_mut();
        let cli = maybe.as_mut().expect("Call wasm_start() first");
        let arg = if cli.state_function.requires_input { Some(input) } else { None };
        cli.submit(arg)
    })
}

#[wasm_bindgen]
pub fn wasm_is_completed() -> bool {
    WASM_CLI.with(|cell| {
        let maybe = cell.borrow();
        maybe.as_ref().map(|c| c.is_completed()).unwrap_or(true)
    })
}

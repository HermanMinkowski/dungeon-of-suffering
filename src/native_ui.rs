use crate::ui::UI;
pub struct NativeUI;

impl UI for NativeUI {
    fn print(&mut self, s: &str) {
        println!("{}", s);
    }

    fn prompt(&mut self, prompt: &str) -> String {
        use std::io::{self, Write};
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf.trim_end().to_string()
    }
}

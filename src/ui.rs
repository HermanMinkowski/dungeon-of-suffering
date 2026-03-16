pub trait UI {
    fn print(&mut self, s: &str);
    fn prompt(&mut self, prompt: &str) -> String;
}

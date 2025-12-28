use rust_i18n::t;

#[derive(Debug)]
pub struct Verbs {
    pub look: String,
    pub take: String,
    pub open: String,
    pub use_verb: String,
    pub push:  String,
    pub go: String,
    pub talk: String,
    pub jump: String,
    pub eat: String,
}

impl Verbs {
    pub fn new() -> Self {
        Self {
            look: t!("verb.look").to_string(),
            take: t!("verb.take").to_string(),
            open: t!("verb.open").to_string(),
            use_verb: t!("verb.use").to_string(),
            push: t!("verb.push").to_string(),
            go: t!("verb.go").to_string(),
            talk: t!("verb.talk").to_string(),
            jump: t!("verb.jump").to_string(),
            eat: t!("verb.eat").to_string(),
        }
    }

    pub fn refresh(&mut self) {
        self.look = t!("verb.look").to_string();
        self.take = t!("verb.take").to_string();
        self.open = t!("verb.open").to_string();
        self.use_verb = t!("verb.use").to_string();
        self.push = t!("verb.push").to_string();
        self.go = t!("verb.go").to_string();
        self.talk = t!("verb.talk").to_string();
        self.jump = t!("verb.jump").to_string();
        self.eat = t!("verb.eat").to_string();
    }
}
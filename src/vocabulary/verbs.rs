use crate::i18n::I18n;

#[derive(Debug)]
pub struct Verbs {
    pub look: String,
    pub take: String,
    pub open: String,
    pub use_verb: String,
    pub push: String,
    pub go: String,
    pub talk: String,
    pub jump: String,
    pub eat: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Verb {    
    Look,
    Take,
    Open,
    Use,
    Push,
    Go,
    Talk,
    Jump,
    Eat,
    #[default]
    Unknown,
}

impl Verbs {
    pub fn new(i18n: &I18n) -> Self {
        Self {
            look: i18n.t("verb-look", None),
            take: i18n.t("verb-take", None),
            open: i18n.t("verb-open", None),
            use_verb: i18n.t("verb-use", None),
            push: i18n.t("verb-push", None),
            go: i18n.t("verb-go", None),
            talk: i18n.t("verb-talk", None),
            jump: i18n.t("verb-jump", None),
            eat: i18n.t("verb-eat", None),
        }
    }

    pub fn refresh(&mut self, i18n: &I18n) {
        self.look = i18n.t("verb-look", None);
        self.take = i18n.t("verb-take", None);
        self.open = i18n.t("verb-open", None);
        self.use_verb = i18n.t("verb-use", None);
        self.push = i18n.t("verb-push", None);
        self.go = i18n.t("verb-go", None);
        self.talk = i18n.t("verb-talk", None);
        self.jump = i18n.t("verb-jump", None);
        self.eat = i18n.t("verb-eat", None);
    }

    pub fn parse(&self, input: &str) -> Verb {
        if input == self.look {
            Verb::Look
        } else if input == self.take {
            Verb::Take
        } else if input == self.open {
            Verb::Open
        } else if input == self.use_verb {
            Verb::Use
        } else if input == self.push {
            Verb::Push
        } else if input == self.go {
            Verb::Go
        } else if input == self.talk {
            Verb::Talk
        } else if input == self.jump {
            Verb::Jump
        } else if input == self.eat {
            Verb::Eat
        } else {
            Verb::Unknown
        }
    }
}

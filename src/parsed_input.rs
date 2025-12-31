use crate::vocabulary::commands::Command;
use crate::vocabulary::objects::Object;
use crate::vocabulary::verbs::Verb;

#[derive(Debug, Default)]
pub struct ParsedInput {
    pub verb: Verb,
    pub command: Command,
    pub object: Object,
    pub raw_object: String,
}

impl ParsedInput {
    pub fn new(verb: Verb, command: Command, object: Object, raw_object: String) -> ParsedInput {
        ParsedInput {
            verb,
            command,
            object,
            raw_object,
        }
    }
}

use crate::{ compiler::Compiler, precedence::Precedence };

pub use self::rules_store::PARSE_RULES;
mod rules_store;
#[derive(Debug)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Compiler)>,
    pub infix: Option<fn(&mut Compiler)>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(
        prefix: Option<fn(&mut Compiler)>,
        infix: Option<fn(&mut Compiler)>,
        precedence: Precedence
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn get_prefix(&self) -> Option<fn(&mut Compiler)> {
        self.prefix
    }

    pub fn get_infix(&self) -> Option<fn(&mut Compiler)> {
        self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }
}

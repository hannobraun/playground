use crate::compiler::tokens::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_token(&mut self, token: Token) -> Token {
        token
    }
}

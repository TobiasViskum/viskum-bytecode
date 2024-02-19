use self::token_type::TokenType;

pub mod token_type;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, start: usize, length: usize, line: usize) -> Self {
        Self { token_type, start, length, line }
    }

    pub fn is(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_lexeme(&self, source: &str) -> String {
        source[self.start..self.start + self.length].to_string()
    }
}

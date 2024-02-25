use crate::{ lexer::Lexer, token::{ Token, token_type::TokenType } };

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    previous: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_token();

        Self { lexer, current: Some(token), previous: None, had_error: false, panic_mode: false }
    }

    pub fn get_previous(&self) -> &Option<Token> {
        &self.previous
    }
    pub fn get_current(&self) -> &Option<Token> {
        &self.current
    }
    pub fn get_had_error(&self) -> bool {
        self.had_error
    }

    pub fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            self.current = Some(self.lexer.scan_token());
            let token = self.current.as_ref().unwrap();
            if let TokenType::TokenError(msg) = token.get_token_type() {
                self.report_error(&msg.clone());
            } else {
                break;
            }
        }
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) {
        let token = self.current.as_ref().unwrap();
        let token_type_current = token.get_token_type();

        if token_type_current == &token_type {
            self.advance();
        } else {
            self.report_error(&format!("Expected {} but got {}", msg, ""));
        }
    }

    pub fn report_error(&mut self, msg: &String) {
        if self.panic_mode {
            return;
        }
        let token = &self.current;
        self.had_error = true;

        self.panic_mode = true;
        if let Some(token) = token {
            let line = token.get_line();
            eprintln!("Print error: {}, at: {}", msg, line)
        } else {
            eprintln!("Print error: {}", msg)
        }
    }

    // self.current = Some(self.lexer.scan_token());
}

use crate::{ lexer::Lexer, token::{ Token, token_type::TokenType } };

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>,
    previous: Option<Token>,
    double_previous_type: Option<TokenType>,
    saved_token: Option<Token>,
    had_error: bool,
    panic_mode: bool,
    can_declare: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let token = lexer.scan_token();

        Self {
            lexer,
            current: Some(token),
            previous: None,
            saved_token: None,
            double_previous_type: None,
            had_error: false,
            panic_mode: false,
            can_declare: true,
        }
    }

    pub fn set_saved_token(&mut self, token: Token) {
        self.saved_token = Some(token);
    }

    pub fn get_double_previous_type(&self) -> &Option<TokenType> {
        &self.double_previous_type
    }

    pub fn set_can_declare(&mut self, can_declare: bool) {
        self.can_declare = can_declare;
    }

    pub fn get_can_declare(&self) -> bool {
        self.can_declare
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

    pub fn get_panic_mode(&self) -> bool {
        self.panic_mode
    }

    pub fn set_panic_mode(&mut self, mode: bool) {
        self.panic_mode = mode;
    }

    pub fn advance(&mut self) {
        if let Some(previous) = &self.previous {
            self.double_previous_type = Some(previous.get_token_type().clone());
        }

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
            eprintln!("Error: {}, at: line {}", msg, line)
        } else {
            eprintln!("Error: {}", msg)
        }
    }

    pub fn report_error_at_saved_token(&mut self, msg: &String) {
        if self.panic_mode {
            return;
        }
        self.had_error = true;
        self.panic_mode = true;
        let token = self.saved_token.as_ref().unwrap();
        let line = token.get_line();
        eprintln!("Error: {}{}, at: line {}", msg, token.get_token_type().as_str(), line)
    }

    // self.current = Some(self.lexer.scan_token());
}

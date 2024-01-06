use crate::{
    token::{ Token, token_type::TokenType::{ self, * } },
    util::general::{ is_digit, is_alphabetic },
};

use super::Lexer;

impl<'a> Lexer<'a> {
    pub(super) fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub(super) fn make_token(&mut self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current - self.start, self.line)
    }

    pub(super) fn error_token(&self, msg: &str) -> Token {
        Token::new(TokenError(String::from(msg)), self.start, self.current - self.start, self.line)
    }

    pub(super) fn advance(&mut self) -> &str {
        let mut i: u8 = 0;
        loop {
            i += 1;
            if i > 4 {
                return "";
            }

            self.current += 1;

            match self.source.get(self.start..self.current) {
                Some(c) => {
                    return c;
                }
                None => {
                    continue;
                }
            };
        }
    }

    pub(super) fn match_char(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.get(self.current..self.current + expected.len()) == Some(expected) {
            self.current += expected.len();
            return true;
        }

        false
    }

    pub(super) fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                " " | "\r" | "\t" => {
                    self.advance();
                }
                "\n" => {
                    self.line += 1;
                    self.advance();
                }
                "/" => {
                    if self.peek_next() == "/" {
                        while self.peek() != "\n" && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    pub(super) fn scan_comment_block(&mut self) {
        let mut comment_count: u8 = 1;

        loop {
            if self.is_at_end() {
                return;
            }

            if self.peek() == "\n" {
                self.line += 1;
            }

            if self.peek() == "/" && self.peek_next() == "*" {
                comment_count += 1;
                self.current += 1;
                self.current += 1;
            }

            if self.peek() == "*" && self.peek_next() == "/" {
                comment_count -= 1;
                self.current += 1;
                self.current += 1;
            }

            if comment_count == 0 {
                return;
            }

            self.current += 1;
        }
    }

    pub(super) fn is_interpolation_start(&self) -> bool {
        self.peek() == "{" && !(self.peek_next() == "{") && !(self.peek_previous() == "{")
    }

    pub(super) fn is_interpolation_end(&self) -> bool {
        self.peek() == "}" && !(self.peek_next() == "}") && !(self.peek_previous() == "}")
    }

    pub(super) fn string(&mut self) -> Token {
        loop {
            if self.peek() == "\"" && !(self.peek_previous() == "\\") {
                let token = self.make_token(TokenString);
                return token;
            }

            if self.peek() == "{" && !(self.peek_next() == "{") && !(self.peek_previous() == "{") {
                return self.make_token(TokenString);
            }
            if self.peek() == "}" && !(self.peek_next() == "}") && !(self.peek_previous() == "}") {
                // Error is handled in the main match statement
                return self.make_token(TokenString);
            }
            if self.peek() == "}" && self.peek_previous() == "}" {
                self.current += 1;
                return self.error_token("Unmatched '}'. To escape use '}}'");
            }

            if self.peek() == "\n" {
                self.line += 1;
            }

            if self.is_at_end() {
                return self.error_token("Unterminated string");
            }

            self.current += 1;
        }
    }

    pub(super) fn number(&mut self) -> Token {
        loop {
            if !is_digit(self.peek()) {
                break;
            }

            if self.is_at_end() {
                return self.error_token("Unterminated number");
            }

            self.current += 1;
        }

        if self.peek() == "." && is_digit(self.peek_next()) {
            self.current += 1;

            loop {
                if !is_digit(self.peek()) {
                    break;
                }

                if self.is_at_end() {
                    return self.error_token("Unterminated number");
                }

                self.current += 1;
            }
        }

        self.make_token(TokenNumber)
    }

    pub(super) fn identifier(&mut self) -> Token {
        loop {
            if !is_alphabetic(self.peek()) && !is_digit(self.peek()) {
                break;
            }

            if self.is_at_end() {
                return self.error_token("Unterminated identifier");
            }

            self.current += 1;
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.get_character(self.start).0 {
            "a" => self.check_keyword(1, 2, "nd", TokenAnd),
            "c" => self.check_keyword(1, 4, "lass", TokenClass),
            "e" => self.check_keyword(1, 3, "lse", TokenElse),
            "f" => {
                if self.current - self.start > 1 {
                    match self.get_character(self.start + 1).0 {
                        "a" => self.check_keyword(2, 3, "lse", TokenFalse),
                        "o" => self.check_keyword(2, 1, "r", TokenFor),
                        "n" => TokenFn,
                        _ => TokenIdentifier,
                    }
                } else {
                    TokenIdentifier
                }
            }
            "i" => self.check_keyword(1, 1, "f", TokenIf),
            "l" => self.check_keyword(1, 2, "et", TokenLet),
            "n" => self.check_keyword(1, 2, "il", TokenNull),
            "o" => self.check_keyword(1, 1, "r", TokenOr),
            "p" => self.check_keyword(1, 4, "rint", TokenPrint),
            "r" => self.check_keyword(1, 5, "eturn", TokenReturn),
            "t" => {
                if self.current - self.start > 1 {
                    match self.get_character(self.start + 1).0 {
                        "h" => self.check_keyword(2, 2, "is", TokenThis),
                        "r" => self.check_keyword(2, 2, "ue", TokenTrue),
                        _ => TokenIdentifier,
                    }
                } else {
                    TokenIdentifier
                }
            }
            "w" => self.check_keyword(1, 4, "hile", TokenWhile),
            _ => TokenIdentifier,
        }
    }

    fn get_character(&self, start: usize) -> (&str, usize) {
        let mut i: usize = 0;
        loop {
            i += 1;
            if i > 4 {
                return ("\0", start + i);
            }

            match self.source.get(start..start + i) {
                Some(c) => {
                    return (c, start + i);
                }
                None => {
                    continue;
                }
            };
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType
    ) -> TokenType {
        if
            self.current - self.start == start + length &&
            self.source.get(self.start + start..self.start + start + length) == Some(rest)
        {
            return token_type;
        }

        TokenIdentifier
    }

    pub(super) fn peek(&self) -> &str {
        self.get_character(self.current).0
    }

    pub(super) fn peek_next(&self) -> &str {
        self.get_character(self.get_character(self.current).1).0
    }

    pub(super) fn peek_previous(&self) -> &str {
        let mut i: usize = 0;

        loop {
            i += 1;
            if i > 4 {
                return "\0";
            }

            match self.source.get(self.current - i..self.current) {
                Some(c) => {
                    return c;
                }
                None => {
                    continue;
                }
            };
        }
    }
}

use crate::{
    token::{ token_type::TokenType::*, Token },
    util::general::{ is_digit, is_alphabetic },
};

mod helper_methods;

pub struct Lexer<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    string_count: u8,
    interpolation_count: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            current: 0,
            start: 0,
            line: 1,
            source: source,
            string_count: 0,
            interpolation_count: 0,
        }
    }

    #[cfg(test)]
    pub fn get_token_names_and_lexemes_vec(&mut self) -> (Vec<String>, Vec<String>) {
        use crate::token::token_type::TokenType;

        let mut token_names = Vec::new();
        let mut token_lexemes = Vec::new();

        loop {
            let token = self.scan_token();

            let token_name = token.get_token_type().as_str().to_string();
            let lexeme = self.source
                .get(token.get_start()..token.get_start() + token.get_length())
                .unwrap()
                .to_string();

            token_names.push(token_name);
            token_lexemes.push(lexeme);

            if token.is(TokenType::TokenEof) {
                break;
            }
        }

        (token_names, token_lexemes)
    }

    pub fn scan_token(&mut self) -> Token {
        if self.string_count == self.interpolation_count {
            self.skip_whitespace();
        }

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenEof);
        }

        if
            self.string_count > 0 &&
            self.string_count != self.interpolation_count &&
            !self.is_interpolation_start() &&
            !self.is_interpolation_end() &&
            !(self.peek() == "\"")
        {
            return self.string();
        }

        let c = self.advance();

        if is_digit(c) {
            return self.number();
        } else if is_alphabetic(c) {
            return self.identifier();
        }

        match c {
            "(" => self.make_token(TokenLeftParen),
            ")" => self.make_token(TokenRightParen),
            "{" => {
                if self.string_count > 0 && !(self.peek() == "{") {
                    self.interpolation_count += 1;
                    return self.make_token(TokenInterpolationStart);
                }
                self.make_token(TokenLeftBrace)
            }
            "}" => {
                if self.interpolation_count > 0 {
                    if self.string_count > self.interpolation_count && self.peek() == "}" {
                        self.current += 1;
                        return self.make_token(TokenRightBrace);
                    } else if self.string_count == self.interpolation_count {
                        self.interpolation_count -= 1;
                        return self.make_token(TokenInterpolationEnd);
                    }
                } else if self.string_count > self.interpolation_count {
                    return self.error_token("Unmatched '}'. To escape use '}}'");
                }

                self.make_token(TokenRightBrace)
            }

            ";" => self.make_token(TokenSemicolon),
            "," => self.make_token(TokenComma),
            "." => self.make_token(TokenDot),
            "-" => self.make_token(TokenMinus),
            "+" => self.make_token(TokenPlus),
            "/" => {
                if self.match_char("*") {
                    self.scan_comment_block();
                    self.scan_token()
                } else {
                    self.make_token(TokenSlash)
                }
            }
            "*" => self.make_token(TokenStar),
            "^" => self.make_token(TokenPower),
            "!" => {
                if self.match_char("=") {
                    self.make_token(TokenBangEqual)
                } else {
                    self.make_token(TokenBang)
                }
            }
            "=" => {
                if self.match_char("=") {
                    self.make_token(TokenEqualEqual)
                } else {
                    self.make_token(TokenEqual)
                }
            }
            "<" => {
                if self.match_char("=") {
                    self.make_token(TokenLessEqual)
                } else {
                    self.make_token(TokenLess)
                }
            }
            ">" => {
                if self.match_char("=") {
                    self.make_token(TokenGreaterEqual)
                } else {
                    self.make_token(TokenGreater)
                }
            }

            "\"" => {
                if self.string_count == 0 {
                    self.string_count += 1;
                    self.make_token(TokenStringStart)
                } else if self.string_count == self.interpolation_count {
                    self.string_count += 1;
                    self.make_token(TokenStringStart)
                } else if self.string_count > self.interpolation_count {
                    self.string_count -= 1;
                    self.make_token(TokenStringEnd)
                } else {
                    // This should maybe change
                    self.string_count -= 1;
                    self.make_token(TokenStringEnd)
                }
            }
            _ => self.error_token("Invalid character"),
        }
    }
}

use crate::{
    opcodes::OpCode,
    parse_rule::{ ParseRule, PARSE_RULES },
    precedence::Precedence,
    token::{ token_type::TokenType, Token },
    value::ValueType,
};
use super::Compiler;
use Precedence::*;
use TokenType::*;

impl<'a> Compiler<'a> {
    pub fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
        if !self.parser.get_had_error() {
            self.compiling_chunk.disassemble("code");
        }
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }

    pub fn expression(&mut self) {
        self.parse_precedence(PrecAssignment)
    }

    fn get_previous(&self) -> &Token {
        self.parser.get_previous().as_ref().unwrap()
    }

    fn get_current(&self) -> &Token {
        self.parser.get_current().as_ref().unwrap()
    }

    pub fn interpolate(&mut self) {
        self.expression();

        self.emit_byte(OpCode::OpInterpolate.into());

        self.parser.consume(TokenInterpolationEnd, "Expected '}' after interpolation");
        self.parser.advance();

        if !(self.get_previous().get_token_type() == &TokenStringEnd) {
            self.string();
            self.emit_byte(OpCode::OpInterpolate.into());
        }

        // println!("previous2: {:?}", self.get_previous());
        // if self.get_current() == &TokenStringEnd {
        //     self.parser.advance();
        // } else {
        //     self.parser.advance();

        //     self.string();
        //     self.emit_byte(OpCode::OpInterpolate.into());
        // }
    }

    pub fn string(&mut self) {
        if self.get_previous().get_token_type() == &TokenStringStart {
            self.parser.advance();
        }

        let previous = self.get_previous();

        if previous.get_token_type() == &TokenInterpolationStart {
            self.emit_constant(ValueType::String("".to_string()), previous.get_line());
            self.interpolate();
            return;
        }

        let string_lexeme = previous.get_lexeme_string(self.source);

        self.emit_constant(ValueType::String(string_lexeme), previous.get_line());

        let new_previous = self.get_previous();
        if new_previous.get_token_type() == &TokenStringEnd {
            self.parser.advance();
        } else {
            self.expression();
        }
    }

    pub fn number(&mut self) {
        let previous = self.get_previous();
        let line = previous.get_line();
        let lexeme = previous.get_lexeme(self.source);

        print!("lexeme: {:?}", lexeme);

        if let Ok(int_value) = lexeme.parse::<i32>() {
            self.emit_constant(ValueType::Int32(int_value), line);
        } else if let Ok(int_value) = lexeme.parse::<i64>() {
            self.emit_constant(ValueType::Int64(int_value), line);
        } else if let Ok(float_value) = lexeme.parse::<f64>() {
            self.emit_constant(ValueType::Float64(float_value), line);
        }
    }

    fn emit_constant(&mut self, value: ValueType, line: usize) {
        let constant = self.make_constant(value, line);
        self.emit_bytes(OpCode::OpConstant.into(), constant)
    }

    fn make_constant(&mut self, value: ValueType, line: usize) -> u8 {
        let constant = self.compiling_chunk.write_constant(value, line);

        // let constant = self.add_constant(current_chunk, value);
        if constant > 255 {
            self.parser.report_error(&"Too many constants in one chunk".to_string());
            return 0;
        }
        constant as u8
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.parser.consume(TokenType::TokenRightParen, "Expected ')' after expression");
    }

    pub fn unary(&mut self) {
        let operator_type = {
            self.parser.get_previous().as_ref().unwrap().get_token_type().clone()
        };

        self.parse_precedence(PrecUnary);

        match operator_type {
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate.into()),
            TokenType::TokenBang => self.emit_byte(OpCode::OpNot.into()),
            _ => {}
        }
    }

    pub fn binary(&mut self) {
        let operator_type = {
            self.parser.get_previous().as_ref().unwrap().get_token_type().clone()
        };

        let parse_rule = self.get_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next());

        match operator_type {
            TokenBangEqual => self.emit_byte(OpCode::OpBangEqual.into()),
            TokenEqualEqual => self.emit_byte(OpCode::OpEqualEqual.into()),
            TokenGreater => self.emit_byte(OpCode::OpGreater.into()),
            TokenGreaterEqual => self.emit_byte(OpCode::OpGreaterEqual.into()),
            TokenLess => self.emit_byte(OpCode::OpLess.into()),
            TokenLessEqual => self.emit_byte(OpCode::OpLessEqual.into()),

            TokenPlus => self.emit_byte(OpCode::OpAdd.into()),
            TokenMinus => self.emit_byte(OpCode::OpSubtract.into()),
            TokenStar => self.emit_byte(OpCode::OpMultiply.into()),
            TokenSlash => self.emit_byte(OpCode::OpDivide.into()),
            TokenPower => self.emit_byte(OpCode::OpPower.into()),
            _ => {}
        }
    }

    pub fn literal(&mut self) {
        let previous_ttype = self.get_previous().get_token_type();
        match previous_ttype {
            TokenFalse => self.emit_byte(OpCode::OpFalse.into()),
            TokenTrue => self.emit_byte(OpCode::OpTrue.into()),
            TokenNull => self.emit_byte(OpCode::OpNull.into()),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();

        let parse_rule = self.get_rule(self.get_previous().get_token_type());

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self);

            loop {
                let current_ttype: TokenType;

                {
                    current_ttype = self.get_current().get_token_type().clone();
                }
                let current_precedence = self.get_rule(&current_ttype).get_precedence();

                if (*current_precedence as usize) < (precedence as usize) {
                    break;
                }
                self.parser.advance();

                let infix_rule = self.get_rule(&current_ttype).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self);
                }
            }
        } else {
            // self.parser.report_error(&"Expected expression".to_string());
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> &ParseRule {
        PARSE_RULES.get(token_type).unwrap()
    }
}

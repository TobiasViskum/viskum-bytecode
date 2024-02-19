use crate::{
    chunk::Value,
    opcodes::OpCode,
    parse_rule::{ ParseRule, PARSE_RULES },
    precedence::Precedence,
    token::token_type::TokenType,
};
use super::Compiler;
use Precedence::*;
use TokenType::*;

impl<'a> Compiler<'a> {
    pub fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(feature = "debug_trace_execution")]
        {
            if !self.parser.get_had_error() {
                self.compiling_chunk.disassemble("code");
            }
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

    pub fn number(&mut self) {
        let previous = self.parser.get_previous().as_ref().unwrap();
        let value = previous.get_lexeme(self.source).parse::<Value>().unwrap();
        let line = previous.get_line();

        self.emit_constant(value, line);
    }

    fn emit_constant(&mut self, value: Value, line: usize) {
        let constant = self.make_constant(value, line);
        self.emit_bytes(OpCode::OpConstant.into(), constant)
    }

    fn make_constant(&mut self, value: Value, line: usize) -> u8 {
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
        let operator_type: TokenType;
        {
            operator_type = self.parser.get_previous().as_ref().unwrap().get_token_type().clone();
        }

        // This errors if I'm not keeping the ref above in its own scope, because it requires a mutable reference to self, but it's already borrowed as immutable
        self.expression();

        self.parse_precedence(PrecUnary);

        match operator_type {
            TokenType::TokenMinus => self.emit_byte(OpCode::OpNegate.into()),
            _ => {}
        }
    }

    pub fn binary(&mut self) {
        let operator_type: TokenType;
        {
            operator_type = self.parser.get_previous().as_ref().unwrap().get_token_type().clone();
        }
        let parse_rule = self.get_rule(&operator_type);

        self.parse_precedence(parse_rule.get_precedence().get_next());

        match operator_type {
            TokenPlus => self.emit_byte(OpCode::OpAdd.into()),
            TokenMinus => self.emit_byte(OpCode::OpSubtract.into()),
            TokenStar => self.emit_byte(OpCode::OpMultiply.into()),
            TokenSlash => self.emit_byte(OpCode::OpDivide.into()),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();

        let parse_rule = self.get_rule(
            self.parser.get_previous().as_ref().unwrap().get_token_type()
        );

        let prefix_rule = parse_rule.get_prefix();

        if let Some(prefix_rule) = prefix_rule {
            prefix_rule(self);

            loop {
                let current_ttype: TokenType;
                {
                    current_ttype = self.parser
                        .get_current()
                        .as_ref()
                        .unwrap()
                        .get_token_type()
                        .clone();
                }
                let current_precedence = self.get_rule(&current_ttype).get_precedence();
                if (precedence as usize) >= (*current_precedence as usize) {
                    break;
                }
                self.parser.advance();

                let infix_rule = self.get_rule(&current_ttype).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self);
                }
            }
        } else {
            self.parser.report_error(&"Expected expression".to_string());
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> &ParseRule {
        PARSE_RULES.get(token_type).unwrap()
    }
}

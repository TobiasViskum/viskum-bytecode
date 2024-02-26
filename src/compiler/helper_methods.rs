use crate::{
    opcodes::OpCode,
    token::{ token_type::TokenType, Token },
    value::{ Value, ValueType, Variable, VariableDefinition },
};
use super::Compiler;

impl<'a> Compiler<'a> {
    fn write_chunk(&mut self, byte: u8, line: usize) {
        self.compiling_chunk.write_byte(byte, line);
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

    pub(super) fn identifier_lookup_constant(&mut self, name: String) -> u8 {
        let constant = self.make_constant(
            Value::VariableLookup(name),
            self.get_previous().get_line()
        );
        constant
    }

    pub(super) fn parse_declaration_name(
        &mut self,
        lexeme: String,
        value_type: ValueType,
        is_mutable: bool
    ) -> u8 {
        let constant = self.make_constant(
            Value::VariableDefinition(VariableDefinition::new(lexeme, value_type, is_mutable)),
            self.get_previous().get_line()
        );
        constant
    }

    pub(super) fn define_variable(&mut self, global_variable_index: u8) {
        self.emit_bytes(OpCode::OpDefineGlobal.into(), global_variable_index);
    }

    pub(super) fn consume_expr_end(&mut self, message: &str) {
        let current = self.get_current();

        match current.get_token_type() {
            TokenType::TokenSemicolon => {
                self.parser.advance();
            }
            TokenType::TokenEof => {}
            _ => {
                let prev_line = self.get_previous().get_line();

                if current.get_line() <= prev_line {
                    self.parser.report_error(&message.to_string());
                }
            }
        }
    }

    pub(super) fn synchronize(&mut self) {
        self.parser.set_panic_mode(false);

        while self.get_current().get_token_type() != &TokenType::TokenEof {
            if self.get_previous().get_token_type() == &TokenType::TokenSemicolon {
                return;
            }

            match self.get_current().get_token_type() {
                | TokenType::TokenClass
                | TokenType::TokenFn
                | TokenType::TokenFor
                | TokenType::TokenIf
                | TokenType::TokenWhile
                | TokenType::TokenPrint
                | TokenType::TokenReturn => {
                    return;
                }
                _ => self.parser.advance(),
            }
        }
    }

    pub(super) fn is_match(&mut self, token_type: &TokenType) -> bool {
        if !(self.get_current().get_token_type() == token_type) {
            false
        } else {
            self.parser.advance();
            true
        }
    }

    pub(super) fn emit_byte(&mut self, byte: u8) {
        let previous = self.parser.get_previous();
        if previous.is_some() {
            let line = previous.as_ref().unwrap().get_line();

            self.write_chunk(byte, line);
        } else {
            self.parser.report_error(&"Failed to emit byte".to_string());
        }
    }

    pub(super) fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub(super) fn emit_constant(&mut self, value: Value, line: usize) {
        let constant = self.make_constant(value, line);
        self.emit_bytes(OpCode::OpConstant.into(), constant)
    }

    pub(super) fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }

    pub(super) fn get_double_previous_type(&self) -> Option<&TokenType> {
        self.parser.get_double_previous_type().as_ref()
    }

    pub(super) fn get_previous(&self) -> &Token {
        self.parser.get_previous().as_ref().unwrap()
    }

    pub(super) fn get_current(&self) -> &Token {
        self.parser.get_current().as_ref().unwrap()
    }

    pub(super) fn set_saved_token(&mut self, token: Token) {
        self.parser.set_saved_token(token);
    }

    pub(super) fn set_can_declare(&mut self, can_declare: bool) {
        self.parser.set_can_declare(can_declare);
    }

    pub(super) fn get_can_declare(&self) -> bool {
        self.parser.get_can_declare()
    }
}

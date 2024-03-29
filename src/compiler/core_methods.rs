use super::Compiler;
use crate::{
    opcodes::OpCode,
    parse_rule::{ ParseRule, PARSE_RULES },
    precedence::Precedence,
    token::token_type::TokenType,
    value::{ Value, ValueType },
};
use Precedence::*;
use TokenType::*;

impl<'a> Compiler<'a> {
    pub fn declaration(&mut self) {
        // let current_ttype = self.get_current();

        // match current_ttype.get_token_type() {
        //     // TokenTypeFloat32 => {
        //     //     self.parser.advance();

        //     //     let identifier_lexeme = self.get_current().get_lexeme(self.source).clone();
        //     // }
        //     TokenIdentifier => {
        //         let identifier_lexeme = current_ttype.get_lexeme(self.source).clone();

        //         self.parser.advance();

        //         let new_current_ttype = self.get_current().get_token_type();
        //         if new_current_ttype == &TokenDeclaration {
        //             self.variable_declaration(identifier_lexeme, ValueType::Dynamic);
        //         } else if new_current_ttype == &TokenEqual {
        //             self.named_variable(identifier_lexeme);
        //         }
        //     }
        //     _ => self.statement(),
        // }

        self.statement();

        if self.parser.get_panic_mode() {
            self.synchronize()
        }
    }

    fn variable_declaration(&mut self, lexeme: String, value_type: ValueType, is_mutable: bool) {
        let declaration_index = self.parse_declaration_name(lexeme, value_type, is_mutable);

        if self.is_match(&TokenDeclaration) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNull.into());
        }

        self.consume_expr_end("Expected ';' after variable declaration");

        self.define_variable(declaration_index)
    }

    fn statement(&mut self) {
        if self.is_match(&TokenPrint) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume_expr_end("Expected ';' after value");
        self.emit_byte(OpCode::OpPrint.into());
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume_expr_end("Expected ';' after value");
        self.emit_byte(OpCode::OpPop.into());
    }

    fn expression(&mut self) {
        self.parse_precedence(PrecAssignment)
    }

    fn named_variable(&mut self, lexeme: String, value_type: ValueType, is_mutable: bool) {
        if self.get_can_declare() && self.get_current().get_token_type() == &TokenDeclaration {
            self.variable_declaration(lexeme, value_type, is_mutable)
        } else if self.get_can_declare() && self.is_match(&TokenEqual) {
            if !(value_type == ValueType::Dynamic || value_type == ValueType::Empty) {
                self.parser.report_error_at_saved_token(
                    &format!("Unexpected type before variable {} is reassigned: ", lexeme)
                );
            } else {
                let variable_index = self.identifier_lookup_constant(lexeme);
                self.expression();
                self.emit_bytes(OpCode::OpSetGlobal.into(), variable_index)
            }
        } else {
            let variable_index = self.identifier_lookup_constant(lexeme);
            self.emit_bytes(OpCode::OpGetGlobal.into(), variable_index)
        }
    }

    pub fn type_keyword(&mut self) {
        let mut is_mutable = false;
        let double_previous_ttype = self.get_double_previous_type();
        if let Some(double_previous_ttype) = double_previous_ttype {
            if double_previous_ttype == &TokenMutable {
                is_mutable = true;
            }
        }

        self.set_saved_token(self.get_previous().clone());

        let parsed_type = self.get_previous().get_token_type().parse_to_type();

        match self.get_current().get_token_type() {
            TokenIdentifier => {
                self.parser.advance();
                let lexeme = self.get_previous().get_lexeme(self.source);
                self.named_variable(lexeme, parsed_type, is_mutable)
            }
            _ => {
                self.parser.report_error(&"Expected identifier after type keyword".to_string());
            }
        }
    }

    pub fn variable(&mut self) {
        let mut is_mutable = false;
        let double_previous_ttype = self.get_double_previous_type();
        if let Some(double_previous_ttype) = double_previous_ttype {
            if double_previous_ttype == &TokenMutable {
                is_mutable = true;
            }
        }

        let lexeme = self.get_previous().get_lexeme(self.source);
        self.named_variable(lexeme, ValueType::Dynamic, is_mutable)
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
    }

    pub fn string(&mut self) {
        if self.get_previous().get_token_type() == &TokenStringStart {
            self.parser.advance();
        }

        if self.get_previous().get_token_type() == &TokenStringEnd {
            self.parser.advance();
            self.emit_constant(Value::String("".to_string()), self.get_previous().get_line());
            return;
        }

        let previous = self.get_previous();
        if previous.get_token_type() == &TokenInterpolationStart {
            self.emit_constant(Value::String("".to_string()), previous.get_line());
            self.interpolate();
            return;
        }

        let string_lexeme = previous.get_lexeme_string(self.source);

        self.emit_constant(Value::String(string_lexeme), previous.get_line());

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

        if let Ok(int_value) = lexeme.parse::<i32>() {
            self.emit_constant(Value::Int32(int_value), line);
        } else if let Ok(int_value) = lexeme.parse::<i64>() {
            self.emit_constant(Value::Int64(int_value), line);
        } else if let Ok(float_value) = lexeme.parse::<f64>() {
            self.emit_constant(Value::Float64(float_value), line);
        }
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
            let can_declare = precedence <= PrecAssignment;
            self.set_can_declare(can_declare);
            prefix_rule(self);

            loop {
                let current_ttype = self.get_current().get_token_type();
                let current_precedence = self.get_rule(current_ttype).get_precedence();

                if (*current_precedence as usize) < (precedence as usize) {
                    break;
                }
                self.parser.advance();

                let infix_rule = self.get_rule(self.get_previous().get_token_type()).get_infix();

                if let Some(infix_rule) = infix_rule {
                    infix_rule(self);
                }

                if can_declare && (self.is_match(&TokenDeclaration) || self.is_match(&TokenEqual)) {
                    self.parser.report_error(&format!("Invalid assignment target"));
                }
            }
        } else {
            println!("No prefix rule found for {:?}", self.get_previous().get_token_type());
            self.parser.report_error(&"Expected expression".to_string());
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> &ParseRule {
        PARSE_RULES.get(token_type).unwrap()
    }

    pub fn parse_next(&mut self) {
        self.parse_precedence(PrecAssignment)
    }

    pub fn empty(&mut self) {}
}

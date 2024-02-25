mod helper_methods;
mod core_methods;

use crate::{ token::token_type::TokenType, chunk::Chunk, parser::Parser };

pub struct Compiler<'a> {
    parser: Parser<'a>,
    source: &'a str,
    compiling_chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        let parser = Parser::new(source);

        Self { parser, source, compiling_chunk: chunk }
    }

    pub fn compile(&mut self) -> bool {
        while !self.is_match(&TokenType::TokenEof) {
            self.declaration();
        }

        self.parser.consume(TokenType::TokenEof, "Expected end of expression");
        self.end_compiler();

        self.parser.get_had_error()
    }

    pub fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
        if !self.parser.get_had_error() {
            self.compiling_chunk.disassemble("code");
        }
    }
}

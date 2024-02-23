mod helper_methods;

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
        self.expression();

        self.parser.consume(TokenType::TokenEof, "Expected end of expression");
        self.end_compiler();

        self.parser.get_had_error()
    }

    pub fn emit_byte(&mut self, byte: u8) {
        let previous = self.parser.get_previous();
        if previous.is_some() {
            let line = previous.as_ref().unwrap().get_line();

            self.write_chunk(byte, line);
        } else {
            self.parser.report_error(&"Failed to emit byte".to_string());
        }
    }

    fn write_chunk(&mut self, byte: u8, line: usize) {
        self.compiling_chunk.write_byte(byte, line);
    }
}

mod helper_methods;

use crate::{ lexer::Lexer, token::token_type::TokenType, chunk::Chunk, parser::Parser };

pub struct Compiler<'a> {
    parser: Parser<'a>,
    source: &'a str,
    compiling_chunk: &'a mut Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str, chunk: &'a mut Chunk) -> Self {
        let parser = Parser::new(source);

        Self { parser: parser, source: source, compiling_chunk: chunk }
    }

    pub fn compile(&mut self) -> bool {
        // self.parser.advance();
        self.expression();

        self.parser.consume(TokenType::TokenEof, "Expected end of expression");
        self.end_compiler();

        println!("Compilation finished");
        println!("Had error: {}", self.parser.get_had_error());
        self.parser.get_had_error()
    }

    pub fn emit_byte(&mut self, byte: u8) {
        let previous = self.parser.get_previous();
        if previous.is_some() {
            let line = previous.as_ref().unwrap().get_line();

            self.write_chunk(byte, line);
        }
    }

    fn write_chunk(&mut self, byte: u8, line: usize) {
        self.compiling_chunk.write_byte(byte, line);
    }

    pub fn compile_old(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);

        let mut line: usize = 0;

        let secs_start = std::time::SystemTime
            ::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        loop {
            let token = lexer.scan_token();

            {
                if line != token.get_line() {
                    print!("{:4} ", token.get_length());
                    line = token.get_line();
                } else {
                    print!("   | ");
                }

                println!(
                    "{:?} '{}'",
                    token.get_token_type().as_str(),
                    source.get(token.get_start()..token.get_start() + token.get_length()).unwrap()
                );
            }

            if token.is(TokenType::TokenEof) {
                break;
            }
        }

        let time =
            std::time::SystemTime
                ::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64() - secs_start;

        println!("{} s", time)
    }
}

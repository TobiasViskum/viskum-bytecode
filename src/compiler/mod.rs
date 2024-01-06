mod helper_methods;

use crate::{ lexer::Lexer, token::token_type::TokenType, chunk::Chunk };

pub struct Compiler<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);

        Self { lexer: lexer }
    }

    pub fn compile(&mut self, chunk: &mut Chunk) -> bool {
        // self.advance();
        // self.expression();
        // self.consume(TokenType::TokenEof, "Expected end of expression");

        true
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

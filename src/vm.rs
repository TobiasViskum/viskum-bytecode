use crate::chunk::{ Chunk, OpCode::{ self, * }, Value };

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
pub struct VM<'a> {
    chunk: Option<&'a mut Chunk>,
    ip: usize,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self { chunk: None, ip: 0 }
    }

    pub fn free(&self) {}

    pub fn interpret(&mut self, chunk: &'a mut Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte().into();
            match instruction {
                OpReturn => {
                    return InterpretResult::Ok;
                }
                OpConstant => {
                    let constant = self.read_constant();
                    println!("{}", constant);
                }
                OpConstantLong => {
                    let constant = self.read_long_constant();
                    println!("{}", constant);
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        let ip = self.ip;
        if let Some(chunk) = &self.chunk {
            chunk.get_code()[ip - 1] as u8
        } else {
            0
        }
    }
    fn read_bytes(&mut self, n: usize) -> usize {
        let mut result = 0;
        for _ in 0..n {
            result = (result << 8) | (self.read_byte() as usize);
        }
        result
    }

    fn read_constant(&mut self) -> Value {
        if self.chunk.is_some() {
            let byte = self.read_byte().into();
            self.chunk.as_ref().unwrap().read_constant(byte)
        } else {
            0.0
        }
    }

    fn read_long_constant(&mut self) -> Value {
        if self.chunk.is_some() {
            let bytes = self.read_bytes(3);
            self.chunk
                .as_ref()
                .unwrap()
                .read_constant(bytes as u16)
        } else {
            0.0
        }
    }
}

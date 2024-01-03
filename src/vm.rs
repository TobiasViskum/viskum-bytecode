use crate::chunk::{ Chunk, Value };
use crate::opcodes::OpCode;

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
    Debug(Value),
}

pub struct VM<'a> {
    chunk: Option<&'a mut Chunk>,
    ip: usize,
    stack: Vec<Value>,
    // stack_top: Value,
}

impl<'a> VM<'a> {
    pub fn new() -> Self {
        Self { chunk: None, ip: 0, stack: Vec::with_capacity(256) }
    }

    pub fn get_stack(&self) -> &Vec<Value> {
        &self.stack
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    pub fn free(&mut self) {
        self.chunk = None;
        self.ip = 0;
    }

    pub fn interpret(&mut self, chunk: &'a mut Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {} ]", slot);
                }
                println!();

                if self.chunk.is_some() {
                    self.chunk.as_ref().unwrap().disassemble_instruction(self.ip);
                }
            }

            let instruction = self.read_byte().into();
            match instruction {
                OpCode::OpReturn => {
                    #[cfg(test)]
                    {
                        return InterpretResult::Debug(self.stack.pop().unwrap());
                    }

                    #[cfg(feature = "debug_trace_execution")]
                    {
                        println!("{}", self.stack.pop().unwrap());
                    }

                    #[allow(unreachable_code)]
                    {
                        return InterpretResult::Ok;
                    }
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }

                OpCode::OpConstantLong => {
                    let constant = self.read_long_constant();
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                OpCode::OpAdd | OpCode::OpSubtract | OpCode::OpMultiply | OpCode::OpDivide => {
                    self.binary_op(instruction);
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

    fn binary_op(&mut self, op: OpCode) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(match op {
            OpCode::OpAdd => a + b,
            OpCode::OpSubtract => a - b,
            OpCode::OpMultiply => a * b,
            OpCode::OpDivide => a / b,
            _ => panic!("Unknown binary op: {}", op.to_string()),
        });
    }
}

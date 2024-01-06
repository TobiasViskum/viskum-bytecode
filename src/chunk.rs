use crate::value::*;
use crate::opcodes::*;

pub type Value = f64;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<(usize, usize)>, // (line, run_length)
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new(), constants: ValueArray::new(), lines: Vec::new() }
    }

    pub fn get_code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn read_constant(&self, index: u16) -> Value {
        self.constants.read(index as usize)
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        if let Some((prev_line, run_length)) = self.lines.last_mut() {
            if *prev_line == line {
                *run_length += 1;
            } else {
                self.lines.push((line, 1));
            }
        } else {
            self.lines.push((line, 1));
        }
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        let index = self.constants.write(value);
        // 0xFF = 255 and is the length of a byte
        if index <= 0xff {
            self.write_byte(OpCode::OpConstant.into(), line); // Operation type "Constant"
            self.write_byte(index as u8, line) // Index of the constant
        } else {
            // If the index is greater than 255, we need to write a
            self.write_byte(OpCode::OpConstantLong.into(), line);
            self.write_byte((index & 0xff) as u8, line);
            self.write_byte(((index >> 8) & 0xff) as u8, line);
            self.write_byte(((index >> 16) & 0xff) as u8, line)
        }
    }

    pub fn free(&mut self) {
        self.code.clear();
        self.constants.free();
    }

    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }

        let instruction = OpCode::from(self.code[offset]);
        match instruction {
            OpCode::OpConstant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::OpConstantLong => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
            OpCode::OpNegate => self.simple_instruction("OP_NEGATE", offset),
            OpCode::OpAdd => self.simple_instruction("OP_ADD", offset),
            OpCode::OpSubtract => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::OpMultiply => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::OpDivide => self.simple_instruction("OP_DIVIDE", offset),
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:<16} {:4} '", name, constant);
        self.constants.print_value(constant);
        println!("'");
        offset + 2
    }

    fn constant_long_instruction(&self, name: &str, offset: usize) -> usize {
        let constant =
            (self.code[offset + 1] as usize) |
            ((self.code[offset + 2] as usize) << 8) |
            ((self.code[offset + 3] as usize) << 16);
        print!("{:<16} {:4} '", name, constant);
        self.constants.print_value(constant as u8);
        println!("'");
        offset + 4
    }

    pub fn get_line(&self, offset: usize) -> usize {
        let mut current = 0;
        for (_, (line, run_length)) in self.lines.iter().enumerate() {
            current += run_length;
            if current >= offset {
                return *line;
            }
        }

        0
    }
}

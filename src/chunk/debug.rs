use super::Chunk;
use crate::opcodes::OpCode;

impl Chunk {
    pub fn disassemble<T: ToString>(&self, name: T) {
        println!("== {} ==", name.to_string());

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
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
}
/* AUTO-GENERATED */
impl Chunk {
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", "");
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.get_line(offset));
        }
        let instruction = OpCode::from(self.code[offset]);
        match instruction {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
            OpCode::OpConstant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::OpConstantLong => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            OpCode::OpNegate => self.simple_instruction("OP_NEGATE", offset),
            OpCode::OpAdd => self.simple_instruction("OP_ADD", offset),
            OpCode::OpSubtract => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::OpMultiply => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::OpDivide => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::OpPower => self.simple_instruction("OP_POWER", offset),
            OpCode::OpNull => self.simple_instruction("OP_NULL", offset),
            OpCode::OpTrue => self.simple_instruction("OP_TRUE", offset),
            OpCode::OpFalse => self.simple_instruction("OP_FALSE", offset),
            OpCode::OpNot => self.simple_instruction("OP_NOT", offset),
            OpCode::OpEqualEqual => self.simple_instruction("OP_EQUAL_EQUAL", offset),
            OpCode::OpGreater => self.simple_instruction("OP_GREATER", offset),
            OpCode::OpGreaterEqual => self.simple_instruction("OP_GREATER_EQUAL", offset),
            OpCode::OpLess => self.simple_instruction("OP_LESS", offset),
            OpCode::OpLessEqual => self.simple_instruction("OP_LESS_EQUAL", offset),
            OpCode::OpBangEqual => self.simple_instruction("OP_BANG_EQUAL", offset),
            OpCode::OpInterpolate => self.simple_instruction("OP_INTERPOLATE", offset),
        }
    }
}
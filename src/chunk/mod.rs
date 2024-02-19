use crate::value::*;
use crate::opcodes::*;

pub type Value = f64;

#[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
mod debug;

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

    pub fn write_constant(&mut self, value: Value, line: usize) -> usize {
        let index = self.constants.write(value);
        // 0xFF = 255 and is the length of a byte
        // if index <= 0xff {
        //     self.write_byte(OpCode::OpConstant.into(), line); // Operation type "Constant"
        //     self.write_byte(index as u8, line); // Index of the constant
        // } else {
        //     // If the index is greater than 255, we need to write a
        //     self.write_byte(OpCode::OpConstantLong.into(), line);
        //     self.write_byte((index & 0xff) as u8, line);
        //     self.write_byte(((index >> 8) & 0xff) as u8, line);
        //     self.write_byte(((index >> 16) & 0xff) as u8, line);
        // }

        index
    }

    pub fn free(&mut self) {
        self.code.clear();
        self.constants.free();
    }
}

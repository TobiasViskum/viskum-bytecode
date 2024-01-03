mod chunk;
mod value;
mod vm;
mod opcodes;
mod tests;

use opcodes::OpCode;
use vm::VM;

use crate::chunk::*;

fn main() {
    let mut vm = VM::new();

    let mut chunk = Chunk::new();

    chunk.write_constant(123344.3, 123);

    chunk.write_byte(OpCode::OpNegate.into(), 123);

    chunk.write_byte(OpCode::OpReturn.into(), 123);

    vm.interpret(&mut chunk);
    vm.free();

    chunk.free();
}

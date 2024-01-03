mod chunk;
mod value;
mod vm;
mod opcodes;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_constant() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        let constant = 123.456;
        chunk.write_constant(constant, 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        assert_eq!(chunk.read_constant(0), constant);

        vm.free();
        chunk.free();
    }

    #[test]
    fn test_op_constant_long() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        // Add enough constants to require OpConstantLong.
        for i in 0..257 {
            chunk.write_constant(i as f64, 1);
        }

        let constant = 123.456;
        chunk.write_constant(constant, 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        assert_eq!(chunk.read_constant(257), constant);

        vm.free();
        chunk.free();
    }
}

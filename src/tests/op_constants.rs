#[cfg(test)]
mod test {
    use crate::{ *, opcodes::OpCode, chunk::Chunk };

    #[test]
    fn test_op_constant() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        let constant = 123.456;
        chunk.write_constant(constant, 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        assert_eq!(chunk.read_constant(0), constant);

        vm.free_chunk();
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

#[cfg(test)]
mod test {
    use crate::{ chunk::{ Chunk, Value }, opcodes::OpCode, value::ValueType, * };

    #[test]
    fn test_op_constant() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        let constant = 123.456;
        chunk.write_constant(ValueType::Float64(constant), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        assert_eq!(chunk.read_constant(0), ValueType::Float64(constant));

        vm.free_chunk();
        chunk.free();
    }

    #[test]
    fn test_op_constant_long() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        // Add enough constants to require OpConstantLong.
        for i in 0..257 {
            chunk.write_constant(ValueType::Float64(i as f64), 1);
        }

        let constant = 123.456;
        chunk.write_constant(ValueType::Float64(constant), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        assert_eq!(chunk.read_constant(257), ValueType::Float64(constant));

        vm.free_vm();
        chunk.free();
    }
}

#[cfg(test)]
mod test {
    use crate::{ *, vm::InterpretResult };

    #[test]
    fn test_op_divide() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        chunk.write_constant(123.0, 1);
        chunk.write_constant(2.0, 1);
        chunk.write_byte(OpCode::OpDivide.into(), 1);
        chunk.write_byte(OpCode::OpNegate.into(), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        let debug_result = vm.interpret(&mut chunk);
        if let InterpretResult::Debug(value) = debug_result {
            assert_eq!(value, -61.5);
        } else {
            panic!("Expected debug result");
        }

        vm.free();
        chunk.free();
    }

    // Write similar tests for the other three opcodes.
    #[test]
    fn test_op_add() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        chunk.write_constant(35.4, 1);
        chunk.write_byte(OpCode::OpNegate.into(), 1);
        chunk.write_constant(8.6, 1);
        chunk.write_byte(OpCode::OpAdd.into(), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        let debug_result = vm.interpret(&mut chunk);
        if let InterpretResult::Debug(value) = debug_result {
            // Round to one decimal place, to avoid floating point errors.
            assert_eq!((value * 100.0).round() / 100.0, -26.8);
        } else {
            panic!("Expected debug result");
        }

        vm.free();
        chunk.free();
    }

    #[test]
    fn test_op_subtract() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        chunk.write_constant(123.0, 1);
        chunk.write_constant(73.0, 1);
        chunk.write_byte(OpCode::OpSubtract.into(), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        let debug_result = vm.interpret(&mut chunk);
        if let InterpretResult::Debug(value) = debug_result {
            assert_eq!(value, 50.0);
        } else {
            panic!("Expected debug result");
        }

        vm.free();
        chunk.free();
    }

    #[test]
    fn test_op_multiply() {
        let mut vm = VM::new();
        let mut chunk = Chunk::new();

        chunk.write_constant(123.0, 1);
        chunk.write_constant(100.0, 1);
        chunk.write_byte(OpCode::OpMultiply.into(), 1);
        chunk.write_byte(OpCode::OpReturn.into(), 1);

        let debug_result = vm.interpret(&mut chunk);
        if let InterpretResult::Debug(value) = debug_result {
            assert_eq!(value, 12300.0);
        } else {
            panic!("Expected debug result");
        }

        vm.free();
        chunk.free();
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_op_divide() {
        let mut vm = VM::new();

        let result = vm.interpret("8 / 3");
        assert_eq!(result, InterpretResult::Debug(8.0 / 3.0));
        vm.free();

        let result = vm.interpret("8 / 2");
        assert_eq!(result, InterpretResult::Debug(4.0));
        vm.free();
    }

    #[test]
    fn test_op_add() {
        let mut vm = VM::new();

        let result = vm.interpret("3 + 3 + 1");
        assert_eq!(result, InterpretResult::Debug(7.0));
        vm.free();

        let result = vm.interpret("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9");
        assert_eq!(result, InterpretResult::Debug(45.0));
        vm.free();
    }

    #[test]
    fn test_op_subtract() {
        let mut vm = VM::new();

        let result = vm.interpret("7 - 3 - 1");
        assert_eq!(result, InterpretResult::Debug(3.0));
        vm.free();

        let result = vm.interpret("1 - 9");
        assert_eq!(result, InterpretResult::Debug(-8.0));
        vm.free();
    }

    #[test]
    fn test_op_multiply() {
        let mut vm = VM::new();

        let result = vm.interpret("8 * 8");
        assert_eq!(result, InterpretResult::Debug(64.0));
        vm.free();

        let result = vm.interpret("2 * 1 * 2");
        assert_eq!(result, InterpretResult::Debug(4.0));
        vm.free();
    }

    #[test]
    fn test_op_power() {
        let mut vm = VM::new();

        let result = vm.interpret("2 ^ 3");
        assert_eq!(result, InterpretResult::Debug(8.0));
        vm.free();

        let result = vm.interpret("2 ^ 4");
        assert_eq!(result, InterpretResult::Debug(16.0));
        vm.free();
    }

    #[test]
    fn test_parentheses() {
        let mut vm = VM::new();

        let result = vm.interpret("8 * (7 + 1)");
        assert_eq!(result, InterpretResult::Debug(64.0));
        vm.free();

        let result = vm.interpret("8 * 7 + 1");
        assert_eq!(result, InterpretResult::Debug(57.0));
        vm.free();

        let result = vm.interpret("8 - (2 - 4)");
        assert_eq!(result, InterpretResult::Debug(10.0));
        vm.free();

        let result = vm.interpret("8 * (7 + 1) / 2");
        assert_eq!(result, InterpretResult::Debug(32.0));
        vm.free();

        let result = vm.interpret("8 * (7 + 1) / 2 - 1");
        assert_eq!(result, InterpretResult::Debug(31.0));
        vm.free();

        let result = vm.interpret("8 * (7 + 1) / 2 - 1 * 2");
        assert_eq!(result, InterpretResult::Debug(30.0));
        vm.free();

        let result = vm.interpret("((8 + 2) * 3) / 2");
        assert_eq!(result, InterpretResult::Debug(15.0));
        vm.free();

        let result = vm.interpret("2 * (3 + 2 - 1)");
        assert_eq!(result, InterpretResult::Debug(8.0));
        vm.free();

        let result = vm.interpret("2 * ()");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free();

        let result = vm.interpret("2 * (3 + 2");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free();

        let result = vm.interpret("2 (3 + 2)");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free();

        // let result = vm.interpret("2 / 0");
        // assert!(matches!(result, InterpretResult::RuntimeError));
        // vm.free();
    }
}

#[cfg(test)]
mod test {
    use crate::{ chunk::Value, value::ValueType, * };

    #[test]
    fn test_negation() {
        let mut vm = VM::new();

        let result = vm.interpret("-1");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(-1.0)));
        vm.free_vm();

        let result = vm.interpret("-1 * 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(-2.0)));
        vm.free_vm();

        let result = vm.interpret("-1 * -2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(2.0)));
        vm.free_vm();

        let result = vm.interpret("-1 * -2 * -3");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(-6.0)));
        vm.free_vm();

        let result = vm.interpret("-1 * -2 * -3 * -4");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(24.0)));
        vm.free_vm();
    }

    #[test]
    fn test_op_divide() {
        let mut vm = VM::new();

        let result = vm.interpret("8 / 3");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(8.0 / 3.0)));
        vm.free_vm();

        let result = vm.interpret("8 / 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(4.0)));
        vm.free_vm();
    }

    #[test]
    fn test_op_add() {
        let mut vm = VM::new();

        let result = vm.interpret("3 + 3 + 1");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(7.0)));
        vm.free_vm();

        let result = vm.interpret("1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(45.0)));
        vm.free_vm();
    }

    #[test]
    fn test_op_subtract() {
        let mut vm = VM::new();

        let result = vm.interpret("7 - 3 - 1");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(3.0)));
        vm.free_vm();

        let result = vm.interpret("1 - 9");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(-8.0)));
        vm.free_vm();
    }

    #[test]
    fn test_op_multiply() {
        let mut vm = VM::new();

        let result = vm.interpret("8 * 8");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(64.0)));
        vm.free_vm();

        let result = vm.interpret("2 * 1 * 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(4.0)));
        vm.free_vm();
    }

    #[test]
    fn test_op_power() {
        let mut vm = VM::new();

        let result = vm.interpret("2 ^ 3");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(8.0)));
        vm.free_vm();

        let result = vm.interpret("2 ^ 4");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(16.0)));
        vm.free_vm();
    }

    #[test]
    fn test_parentheses() {
        let mut vm = VM::new();

        let result = vm.interpret("8 * (7 + 1)");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(64.0)));
        vm.free_vm();

        let result = vm.interpret("8 * 7 + 1");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(57.0)));
        vm.free_vm();

        let result = vm.interpret("8 - (2 - 4)");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(10.0)));
        vm.free_vm();

        let result = vm.interpret("8 * (7 + 1) / 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(32.0)));
        vm.free_vm();

        let result = vm.interpret("8 * (7 + 1) / 2 - 1");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(31.0)));
        vm.free_vm();

        let result = vm.interpret("8 * (7 + 1) / 2 - 1 * 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(30.0)));
        vm.free_vm();

        let result = vm.interpret("((8 + 2) * 3) / 2");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(15.0)));
        vm.free_vm();

        let result = vm.interpret("2 * (3 + 2 - 1)");
        assert_eq!(result, InterpretResult::Debug(ValueType::Float64(8.0)));
        vm.free_vm();

        let result = vm.interpret("2 * ()");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free_vm();

        let result = vm.interpret("2 * (3 + 2");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free_vm();

        let result = vm.interpret("2 (3 + 2)");
        assert!(matches!(result, InterpretResult::CompileError));
        vm.free_vm();

        // let result = vm.interpret("2 / 0");
        // assert!(matches!(result, InterpretResult::RuntimeError)));
        // vm.free_vm();
    }
}

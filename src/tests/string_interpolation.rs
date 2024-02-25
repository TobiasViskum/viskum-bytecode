#[cfg(test)]
mod test {
    use crate::{ value::ValueType, vm::{ InterpretResult, VM } };

    #[test]
    fn test_basic_string_interpolation() {
        let mut vm = VM::new();

        let result = vm.interpret("\"{1}\"");
        assert_eq!(result, InterpretResult::Debug(ValueType::String("1".to_string())));
        vm.free_vm();

        let result = vm.interpret("\"{(4 * 0 + 1 * 3 - 1) * 2}\"");
        assert_eq!(result, InterpretResult::Debug(ValueType::String("4".to_string())));
        vm.free_vm();

        let result = vm.interpret("\"1 + 1 equals {1 + 1}\"");
        assert_eq!(result, InterpretResult::Debug(ValueType::String("1 + 1 equals 2".to_string())));
        vm.free_vm();

        let result = vm.interpret("\"1 + 1 equals {1 + 1} and 2 + 2 equals {2 + 2}\"");
        assert_eq!(
            result,
            InterpretResult::Debug(
                ValueType::String("1 + 1 equals 2 and 2 + 2 equals 4".to_string())
            )
        );
        vm.free_vm();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn compare_equal_equal() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 == 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 == 7");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(false)));
        vm.free_vm();
    }

    #[test]
    fn compare_not_equal() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 != 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(false)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 != 7");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();
    }

    #[test]
    fn compare_less_than() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 < 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(false)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 < 7");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();
    }

    #[test]
    fn compare_less_than_or_equal() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 <= 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 <= 7");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();
    }

    #[test]
    fn compare_greater_than() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 > 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(false)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 > 5");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();
    }

    #[test]
    fn compare_greater_than_or_equal() {
        use crate::{ value::Value, vm::{ InterpretResult, VM } };

        let mut vm = VM::new();
        let result = vm.interpret("2 * 3 >= 6");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();

        let result = vm.interpret("2 * 3 >= 5");
        assert_eq!(result, InterpretResult::Debug(Value::Bool(true)));
        vm.free_vm();
    }
}
